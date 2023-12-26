//! NEON-optimized implementation for aarch64 CPUs.
//!
//! Adapted from the Crypto++ `chacha_simd` implementation by Jack Lloyd and
//! Jeffrey Walton (public domain).

use crate::{Rounds, STATE_WORDS};
use core::{arch::aarch64::*, marker::PhantomData};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U4, U64},
    BlockSizeUser, ParBlocks, ParBlocksSizeUser, StreamBackend, StreamClosure,
};

struct Backend<R: Rounds> {
    state: [uint32x4_t; 4],
    ctrs: [uint32x4_t; 4],
    _pd: PhantomData<R>,
}

impl<R: Rounds> Backend<R> {
    #[inline]
    unsafe fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        let state = [
            vld1q_u32(state.as_ptr().offset(0)),
            vld1q_u32(state.as_ptr().offset(4)),
            vld1q_u32(state.as_ptr().offset(8)),
            vld1q_u32(state.as_ptr().offset(12)),
        ];
        let ctrs = [
            vld1q_u32([1, 0, 0, 0].as_ptr()),
            vld1q_u32([2, 0, 0, 0].as_ptr()),
            vld1q_u32([3, 0, 0, 0].as_ptr()),
            vld1q_u32([4, 0, 0, 0].as_ptr()),
        ];
        Backend::<R> {
            state,
            ctrs,
            _pd: PhantomData,
        }
    }
}

#[inline]
#[cfg(feature = "cipher")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn inner<R, F>(state: &mut [u32; STATE_WORDS], f: F)
where
    R: Rounds,
    F: StreamClosure<BlockSize = U64>,
{
    let mut backend = Backend::<R>::new(state);

    f.call(&mut backend);

    vst1q_u32(state.as_mut_ptr().offset(12), backend.state[3]);
}

#[inline]
#[cfg(feature = "rand_core")]
#[target_feature(enable = "neon")]
/// Sets up backend and blindly writes 4 blocks to dest_ptr.
pub(crate) unsafe fn rng_inner<R, V>(
    state: &mut [u32; STATE_WORDS],
    mut dest_ptr: *mut u8,
    num_blocks: usize,
) where
    R: Rounds,
    V: Variant,
{
    let mut backend = Backend::<R>::new(state);

    let num_chunks = num_blocks >> 2;
    let remaining = num_blocks & 0x03;

    for _chunk in 0..num_chunks {
        backend.write_par_ks_blocks(dest_ptr, 4);
        dest_ptr = dest_ptr.add(256);
    }
    if remaining > 0 {
        backend.write_par_ks_blocks(dest_ptr, remaining);
    }

    vst1q_u32(state.as_mut_ptr().offset(12), backend.state[3]);
}

#[cfg(feature = "cipher")]
impl<R: Rounds> BlockSizeUser for Backend<R> {
    type BlockSize = U64;
}
#[cfg(feature = "cipher")]
impl<R: Rounds> ParBlocksSizeUser for Backend<R> {
    type ParBlocksSize = U4;
}

#[cfg(feature = "cipher")]
impl<R: Rounds> StreamBackend for Backend<R> {
    #[inline(always)]
    fn gen_ks_block(&mut self, block: &mut Block) {
        // SAFETY: Block is a 64-byte array
        unsafe { self.write_par_ks_blocks(block.as_mut_ptr(), 1) }
    }

    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut ParBlocks<Self>) {
        // SAFETY: `ParBlocks` is a 256-byte 2D array.
        unsafe { self.write_par_ks_blocks(blocks.as_mut_ptr() as *mut u8, 4) }
    }
}

macro_rules! add64 {
    ($a:expr, $b:expr) => {
        vreinterpretq_u32_u64(vaddq_u64(
            vreinterpretq_u64_u32($a),
            vreinterpretq_u64_u32($b),
        ))
    };
}

macro_rules! rotate_left {
    ($v:expr, 8) => {{
        let maskb = [3u8, 0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 10, 15, 12, 13, 14];
        let mask = vld1q_u8(maskb.as_ptr());

        $v = vreinterpretq_u32_u8(vqtbl1q_u8(vreinterpretq_u8_u32($v), mask))
    }};
    ($v:expr, 16) => {
        $v = vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32($v)))
    };
    ($v:expr, $r:literal) => {
        $v = vorrq_u32(vshlq_n_u32($v, $r), vshrq_n_u32($v, 32 - $r))
    };
}

macro_rules! extract {
    ($v:expr, $s:literal) => {
        $v = vextq_u32($v, $v, $s)
    };
}

/// Evaluates to `a = a + b`, where the operands are u32x4s
macro_rules! add_assign_vec {
    ($a:expr, $b:expr) => {
        $a = vaddq_u32($a, $b)
    };
}

impl<R: Rounds> Backend<R> {
    #[inline(always)]
    /// Generates `num_blocks` blocks and blindly writes them to `dest_ptr`
    ///
    /// `num_blocks` must be greater than 0, and less than or equal to 4.
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could produce undefined behavior
    unsafe fn write_par_ks_blocks(&mut self, mut dest_ptr: *mut u8, num_blocks: usize) {
        assert!(
            num_blocks <= 4 && num_blocks > 0,
            "neon::write_par_ks_blocks() error: num_blocks must be: 1 <= num_blocks <= 4"
        );

        // these are the output blocks, and they cannot persist as a member of Backend
        // without producing incorrect values (eventually, in fill_bytes())
        let mut blocks = [
            [self.state[0], self.state[1], self.state[2], self.state[3]],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[0])],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[1])],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[2])],
        ];

        for _ in 0..R::COUNT {
            double_quarter_round(&mut blocks);
        }

        for block in 0..num_blocks {
            // add state to block
            for state_row in 0..4 {
                add_assign_vec!(blocks[block][state_row], self.state[state_row]);
            }
            if block > 0 {
                blocks[block][3] = add64!(blocks[block][3], self.ctrs[block - 1]);
            }
            // write blocks to pointer
            for state_row in 0..4 {
                vst1q_u8(
                    dest_ptr.offset(state_row << 4),
                    vreinterpretq_u8_u32(blocks[block][state_row as usize]),
                );
            }
            dest_ptr = dest_ptr.add(64);
        }
        self.state[3] = add64!(self.state[3], self.ctrs[num_blocks - 1]);
    }
}

#[inline]
unsafe fn double_quarter_round(blocks: &mut [[uint32x4_t; 4]; 4]) {
    add_xor_rot(blocks);
    rows_to_cols(blocks);
    add_xor_rot(blocks);
    cols_to_rows(blocks);
}

#[inline]
unsafe fn add_xor_rot(blocks: &mut [[uint32x4_t; 4]; 4]) {
    /// Evaluates to `a = a ^ b`, where the operands are u32x4s
    macro_rules! xor_assign_vec {
        ($a:expr, $b:expr) => {
            $a = veorq_u32($a, $b)
        };
    }
    for block in blocks.iter_mut() {
        // this part of the code cannot be reduced much more without having
        // to deal with some problems regarding `rotate_left` requiring the second
        // argument to be a const, and const arrays cannot be indexed by non-consts
        add_assign_vec!(block[0], block[1]);
        xor_assign_vec!(block[3], block[0]);
        rotate_left!(block[3], 16);

        add_assign_vec!(block[2], block[3]);
        xor_assign_vec!(block[1], block[2]);
        rotate_left!(block[1], 12);

        add_assign_vec!(block[0], block[1]);
        xor_assign_vec!(block[3], block[0]);
        rotate_left!(block[3], 8);

        add_assign_vec!(block[2], block[3]);
        xor_assign_vec!(block[1], block[2]);
        rotate_left!(block[1], 7);
    }
}

#[inline]
unsafe fn rows_to_cols(blocks: &mut [[uint32x4_t; 4]; 4]) {
    for block in blocks.iter_mut() {
        extract!(block[1], 1);
        extract!(block[2], 2);
        extract!(block[3], 3);
    }
}

#[inline]
unsafe fn cols_to_rows(blocks: &mut [[uint32x4_t; 4]; 4]) {
    for block in blocks.iter_mut() {
        extract!(block[1], 3);
        extract!(block[2], 2);
        extract!(block[3], 1);
    }
}
