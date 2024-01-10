//! NEON-optimized implementation for aarch64 CPUs.
//!
//! Adapted from the Crypto++ `chacha_simd` implementation by Jack Lloyd and
//! Jeffrey Walton (public domain).

use crate::{Rounds, STATE_WORDS, CONSTANTS, Variant, backends::BackendType, impl_chacha_core};
use core::{arch::aarch64::*, marker::PhantomData};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U4, U64},
    BlockSizeUser, ParBlocks, ParBlocksSizeUser, StreamBackend, StreamClosure,
    StreamCipherCore,
    StreamCipherSeekCore
};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone)]
pub struct ChaChaCore<R: Rounds, V: Variant> {
    pub(crate) state: [u32; STATE_WORDS],
    backend: Backend<R, V>
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> Drop for ChaChaCore<R, V> {
    fn drop(&mut self) {
        self.state.zeroize();
    }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> ZeroizeOnDrop for ChaChaCore<R, V> {}

impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
    #[inline]
    pub fn new(key: &[u8; 32], iv: &[u8]) -> Self {
        let mut state = [0u32; STATE_WORDS];
        state[0..4].copy_from_slice(&CONSTANTS);
        let key_chunks = key.chunks_exact(4);
        for (val, chunk) in state[4..12].iter_mut().zip(key_chunks) {
            *val = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        let iv_chunks = iv.as_ref().chunks_exact(4);
        for (val, chunk) in state[V::NONCE_INDEX..16].iter_mut().zip(iv_chunks) {
            *val = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        Self {
            state,
            backend: Backend::new(&state)
        }
    }

    #[inline]
    pub fn update_state(&mut self) {
        self.backend.update_state(&self.state)
    }

    #[inline]
    #[cfg(feature = "rng")]
    pub(crate) fn rng_inner(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        self.backend.rng_inner(dest_ptr, num_blocks);
        self.state[12] = self.state[12].wrapping_add(num_blocks as u32);
    }
}

impl_chacha_core!();

#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> StreamCipherCore for ChaChaCore<R, V> {
    #[inline(always)]
    fn remaining_blocks(&self) -> Option<usize> {
        let rem = u32::MAX - self.get_block_pos();
        rem.try_into().ok()
    }

    /// Generate output, overwriting data already in the buffer.
    #[inline]
    fn process_with_backend(&mut self, f: impl StreamClosure<BlockSize = U64>) {
        unsafe {
            f.call(self)
        }
    }
}

#[derive(Clone)]
struct Backend<R: Rounds, V: Variant> {
    state: [uint32x4_t; 4],
    ctrs: [uint32x4_t; 4],
    results: [[uint32x4_t; 4]; 4],
    block: usize,
    _pd: PhantomData<R>,
    _variant: PhantomData<V>
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> Drop for Backend<R, V> {
    fn drop(&mut self) {
        self.state.zeroize();
    }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> ZeroizeOnDrop for Backend<R, V> {}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> Zeroize for Backend<R, V> {
    fn zeroize(&mut self) {
        self.results.zeroize()
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

impl<R: Rounds, V: Variant> BackendType for Backend<R, V> {
    const PAR_BLOCKS: usize = 4;
    #[inline]
    fn new(state: &[u32; STATE_WORDS]) -> Self {
        unsafe {
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
            Backend::<R, V> {
                state,
                ctrs,
                results: [
                    [state[0], state[1], state[2], state[3]],
                    [state[0], state[1], state[2], add64!(state[3], ctrs[0])],
                    [state[0], state[1], state[2], add64!(state[3], ctrs[1])],
                    [state[0], state[1], state[2], add64!(state[3], ctrs[2])],
                ],
                block: 4,
                _pd: PhantomData,
                _variant: PhantomData
            }
        }
    }

    #[inline]
    fn update_state(&mut self, state: &[u32]) {
        unsafe {
            self.state[3] = vld1q_u32(state.as_ptr().offset(12));
        }
        self.block = 4;
    }

    #[inline]
    fn increment_counter(&mut self, amount: i32) {
        debug_assert!(amount > 0 && amount <= Self::PAR_BLOCKS.try_into().unwrap());
        unsafe {
            self.state[3] = add64!(self.state[3], self.ctrs[amount as usize - 1]);
        }
    }

    //#[inline(always)]
    /// Generates `num_blocks` blocks and blindly writes them to `dest_ptr`
    ///
    /// `num_blocks` must be greater than 0, and less than or equal to 4.
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could cause a segmentation fault and/or undesired 
    /// behavior
    unsafe fn write_ks_blocks(&mut self, mut dest_ptr: *mut u8, mut num_blocks: usize) {
        if self.block >= Self::PAR_BLOCKS {
            self.rounds();
            self.block = 0;
        }

        // using a saturating add because this could overflow. Very small chance though
        let max_block_index = Self::PAR_BLOCKS.min(self.block.saturating_add(num_blocks));
        for block in self.block..max_block_index {
            // write blocks to pointer
            for state_row in 0..4 {
                vst1q_u8(
                    dest_ptr.offset(state_row << 4),
                    vreinterpretq_u8_u32(self.results[block][state_row as usize]),
                );
            }
            dest_ptr = dest_ptr.add(64);
        }
        num_blocks -= max_block_index - self.block;
        self.block = max_block_index;
        if num_blocks > 0 {
            self.write_ks_blocks(dest_ptr, num_blocks)
        }
    }

    #[cfg(feature = "rng")]
    #[inline]
    fn rng_inner(&mut self, mut dest_ptr: *mut u8, mut num_blocks: usize) {
        // limiting recursion depth to a maximum of 2 recursive calls to try 
        // to reduce memory usage
        unsafe {
            while num_blocks > 4 {
                self.write_ks_blocks(dest_ptr, 4);
                dest_ptr = dest_ptr.add(256);
                num_blocks -= 4;
            }
            if num_blocks > 0 {
                self.write_ks_blocks(dest_ptr, num_blocks);
            }
        }
    }
}

impl<R: Rounds, V: Variant> Backend<R, V> {
    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn rounds(&mut self) {
        self.results = [
            [self.state[0], self.state[1], self.state[2], self.state[3]],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[0])],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[1])],
            [self.state[0], self.state[1], self.state[2], add64!(self.state[3], self.ctrs[2])],
        ];

        for _ in 0..R::COUNT {
            double_quarter_round(&mut self.results);
        }

        for block in 0..4 {
            // add state to block
            for state_row in 0..4 {
                add_assign_vec!(self.results[block][state_row], self.state[state_row]);
            }
            if block > 0 {
                self.results[block][3] = add64!(self.results[block][3], self.ctrs[block - 1]);
            }
        }
        
        self.increment_counter(Self::PAR_BLOCKS as i32);
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> ParBlocksSizeUser for ChaChaCore<R, V> {
    type ParBlocksSize = U4;
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamBackend for ChaChaCore<R, V> {
    #[inline(always)]
    fn gen_ks_block(&mut self, block: &mut Block) {
        // SAFETY: Block is a 64-byte array
        unsafe { self.backend.write_ks_blocks(block.as_mut_ptr(), 1); }
        self.state[12] = self.state[12].wrapping_add(1);
    }

    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut ParBlocks<Self>) {
        // SAFETY: `ParBlocks` is a 256-byte 2D array.
        unsafe { self.backend.write_ks_blocks(blocks.as_mut_ptr() as *mut u8, 4); }
        self.state[12] = self.state[12].wrapping_add(4);
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
