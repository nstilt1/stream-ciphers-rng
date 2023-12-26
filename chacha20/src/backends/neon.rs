//! NEON-optimized implementation for aarch64 CPUs.
//!
//! Adapted from the Crypto++ `chacha_simd` implementation by Jack Lloyd and
//! Jeffrey Walton (public domain).

use crate::{Rounds, STATE_WORDS};
use core::{arch::aarch64::*, marker::PhantomData};

#[cfg(feature = "rand_core")]
use crate::{variants::Variant, ChaChaCore};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U4, U64},
    BlockSizeUser, ParBlocks, ParBlocksSizeUser, StreamBackend, StreamClosure,
};

struct Backend<R: Rounds> {
    state: [uint32x4_t; 4],
    blocks: [[uint32x4_t; 4]; 4],
    ctrs: [uint32x4_t; 4],
    _pd: PhantomData<R>,
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
        let blocks = [
            [state[0], state[1], state[2], state[3]],
            [state[0], state[1], state[2], add64!(state[3], ctrs[0])],
            [state[0], state[1], state[2], add64!(state[3], ctrs[1])],
            [state[0], state[1], state[2], add64!(state[3], ctrs[2])]
        ];
        Backend::<R> {
            state,
            blocks,
            ctrs,
            _pd: PhantomData,
        }
    }

    #[inline]
    unsafe fn add_xor_rot(&mut self) {
        macro_rules! add_word {
            ($result:expr, $add_amount:expr) => {
                $result = vaddq_u32($result, $add_amount)
            };
        }
        macro_rules! xor_word {
            ($result:expr, $xor_val:expr) => {
                $result = veorq_u32($result, $xor_val)
            };
        }
        for block in self.blocks.iter_mut() {
            // this part of the code cannot be reduced much more without having 
            // to deal with some problems regarding `rotate_left` requiring the second 
            // argument to be a const, and const arrays cannot be indexed by non-consts
            add_word!(block[0], block[1]);
            xor_word!(block[3], block[0]);
            rotate_left!(block[3], 16);

            add_word!(block[2], block[3]);
            xor_word!(block[1], block[2]);
            rotate_left!(block[1], 12);

            add_word!(block[0], block[1]);
            xor_word!(block[3], block[0]);
            rotate_left!(block[3], 8);

            add_word!(block[2], block[3]);
            xor_word!(block[1], block[2]);
            rotate_left!(block[1], 7);
        }
    }

    #[inline] 
    unsafe fn rows_to_cols(&mut self) {
        for block in self.blocks.iter_mut() {
            extract!(block[1], 1);
            extract!(block[2], 2);
            extract!(block[3], 3);
        }
    }

    #[inline] 
    unsafe fn cols_to_rows(&mut self) {
        for block in self.blocks.iter_mut() {
            extract!(block[1], 3);
            extract!(block[2], 2);
            extract!(block[3], 1);
        }
    }

    #[inline]
    unsafe fn double_quarter_round(&mut self) {
        self.add_xor_rot();
        self.rows_to_cols();
        self.add_xor_rot();
        self.cols_to_rows();
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
    core: &mut ChaChaCore<R, V>,
    mut dest_ptr: *mut u8,
    num_blocks: usize,
) where
    R: Rounds,
    V: Variant,
{
    let mut backend = Backend::<R>::new(&mut core.state);

    let num_chunks = num_blocks >> 2;
    let remaining = num_blocks & 0x03;

    for _chunk in 0..num_chunks {
        backend.write_par_ks_blocks(dest_ptr, 4);
        dest_ptr = dest_ptr.add(256);
    }
    if remaining > 0 {
        backend.write_par_ks_blocks(dest_ptr, remaining);
    }

    vst1q_u32(core.state.as_mut_ptr().offset(12), backend.state[3]);
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
        unsafe {
            self.write_par_ks_blocks(block.as_mut_ptr(), 1)
        }
    }

    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut ParBlocks<Self>) {
        // SAFETY: `ParBlocks` is a 256-byte 2D array.
        unsafe {
            self.write_par_ks_blocks(blocks.as_mut_ptr() as *mut u8, 4)
        }
    }
}

impl<R: Rounds> Backend<R> {
    #[inline(always)]
    /// Generates `num_blocks` blocks and blindly writes them to `dest_ptr`
    ///
    /// `num_blocks` must be less than or equal to 4.
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could produce undefined behavior
    unsafe fn write_par_ks_blocks(&mut self, mut dest_ptr: *mut u8, num_blocks: usize) {
        assert!(
            num_blocks <= 4 && num_blocks > 0,
            "neon::write_par_ks_blocks() error: num_blocks must be: 1 <= num_blocks <= 4"
        );

        for _ in 0..R::COUNT {
            self.double_quarter_round();
        }

        self.blocks[0][0] = vaddq_u32(self.blocks[0][0], self.state[0]);
        self.blocks[0][1] = vaddq_u32(self.blocks[0][1], self.state[1]);
        self.blocks[0][2] = vaddq_u32(self.blocks[0][2], self.state[2]);
        self.blocks[0][3] = vaddq_u32(self.blocks[0][3], self.state[3]);

        self.blocks[1][0] = vaddq_u32(self.blocks[1][0], self.state[0]);
        self.blocks[1][1] = vaddq_u32(self.blocks[1][1], self.state[1]);
        self.blocks[1][2] = vaddq_u32(self.blocks[1][2], self.state[2]);
        self.blocks[1][3] = vaddq_u32(self.blocks[1][3], self.state[3]);
        self.blocks[1][3] = add64!(self.blocks[1][3], self.ctrs[0]);

        self.blocks[2][0] = vaddq_u32(self.blocks[2][0], self.state[0]);
        self.blocks[2][1] = vaddq_u32(self.blocks[2][1], self.state[1]);
        self.blocks[2][2] = vaddq_u32(self.blocks[2][2], self.state[2]);
        self.blocks[2][3] = vaddq_u32(self.blocks[2][3], self.state[3]);
        self.blocks[2][3] = add64!(self.blocks[2][3], self.ctrs[1]);

        self.blocks[3][0] = vaddq_u32(self.blocks[3][0], self.state[0]);
        self.blocks[3][1] = vaddq_u32(self.blocks[3][1], self.state[1]);
        self.blocks[3][2] = vaddq_u32(self.blocks[3][2], self.state[2]);
        self.blocks[3][3] = vaddq_u32(self.blocks[3][3], self.state[3]);
        self.blocks[3][3] = add64!(self.blocks[3][3], self.ctrs[2]);

        for block in 0..num_blocks {

            // write blocks to pointer
            for col in 0..4 {
                vst1q_u8(dest_ptr.offset(col << 4), vreinterpretq_u8_u32(self.blocks[block][col as usize]));
            }
            dest_ptr = dest_ptr.add(64);
        }
        self.state[3] = add64!(self.state[3], self.ctrs[num_blocks - 1]);
    }
}