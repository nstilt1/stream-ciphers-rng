//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use core::marker::PhantomData;

use crate::{variants::Variant, Rounds, STATE_WORDS};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U1, U64},
    BlockSizeUser, ParBlocksSizeUser, StreamBackend
};

use super::BackendType;

#[derive(Clone)]
pub(crate) struct Backend<R: Rounds, V: Variant>{
    state: [u32; 16],
    _r: PhantomData<R>,
    _variant: PhantomData<V>
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for Backend<R, V> {
    type BlockSize = U64;
}
#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> ParBlocksSizeUser for Backend<R, V> {
    type ParBlocksSize = U1;
}

impl<R: Rounds, V: Variant> BackendType for Backend<R, V> {
    const PAR_BLOCKS: usize = 1;

    fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        Self {
            state: *state,
            _r: PhantomData,
            _variant: PhantomData
        }
    }
    #[inline(always)]
    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could produce undefined behavior
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        let mut block_ptr = dest_ptr as *mut u32;
        for _i in 0..num_blocks {
            let res = run_rounds::<R>(&self.state);
            self.increment_counter(1);

            for val in res.iter() {
                block_ptr.write_unaligned(val.to_le());
                block_ptr = block_ptr.add(1);
            }
        }
    }

    #[cfg(feature = "rand_core")]
    fn set_nonce(&mut self, nonce: [u32; 3]) {
        for (state, val) in self.state.iter_mut().zip(nonce.iter()) {
            *state = val.to_le()
        }
    }

    #[cfg(feature = "rand_core")]
    fn get_nonce(&self) -> [u32; 3] {
        let mut result = [0u32; 3];
        result.copy_from_slice(&self.state[13..16]);
        result
    }

    #[cfg(feature = "rand_core")]
    fn get_seed(&self) -> [u32; 8] {
        let mut result = [0u32; 8];
        result.copy_from_slice(&self.state[4..12]);
        result
    }

    fn increment_counter(&mut self, amount: i32) {
        self.state[12] = self.state[12].wrapping_add(amount as u32);
    }

    fn get_block_pos(&self) -> u32 {
        self.state[12]
    }

    fn set_block_pos(&mut self, pos: u32) {
        self.state[12] = pos
    }
}

#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> StreamBackend for Backend<R, V> {
    #[inline(always)]
    /// Writes a single block to `block`
    fn gen_ks_block(&mut self, block: &mut Block) {
        // SAFETY: `Block` is a 64-byte array
        unsafe {
            self.write_ks_blocks(block.as_mut_ptr(), 1);
        }
    }
}

#[inline(always)]
fn run_rounds<R: Rounds>(state: &[u32; STATE_WORDS]) -> [u32; STATE_WORDS] {
    let mut res = *state;

    for _ in 0..R::COUNT {
        // column rounds
        quarter_round(0, 4, 8, 12, &mut res);
        quarter_round(1, 5, 9, 13, &mut res);
        quarter_round(2, 6, 10, 14, &mut res);
        quarter_round(3, 7, 11, 15, &mut res);

        // diagonal rounds
        quarter_round(0, 5, 10, 15, &mut res);
        quarter_round(1, 6, 11, 12, &mut res);
        quarter_round(2, 7, 8, 13, &mut res);
        quarter_round(3, 4, 9, 14, &mut res);
    }

    for (s1, s0) in res.iter_mut().zip(state.iter()) {
        *s1 = s1.wrapping_add(*s0);
    }
    res
}

/// The ChaCha20 quarter round function
fn quarter_round(a: usize, b: usize, c: usize, d: usize, state: &mut [u32; STATE_WORDS]) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);

    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
}
