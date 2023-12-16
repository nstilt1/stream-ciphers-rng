//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use crate::{variants::Variant, ChaChaCore, Rounds, STATE_WORDS};

#[cfg(feature = "cipher")]
use crate::chacha::Block;
#[cfg(feature = "cipher")]
use cipher::{
    consts::{U1, U64},
    BlockSizeUser, ParBlocksSizeUser, StreamBackend,
};

pub(crate) struct Backend<'a, R: Rounds, V: Variant>(pub(crate) &'a mut ChaChaCore<R, V>);

#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> BlockSizeUser for Backend<'a, R, V> {
    type BlockSize = U64;
}
#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> ParBlocksSizeUser for Backend<'a, R, V> {
    type ParBlocksSize = U1;
}

impl<'a, R: Rounds, V: Variant> Backend<'a, R, V> {
    #[inline(always)]
    /// Generates a single keystream block and writes it to a pointer
    pub(crate) unsafe fn write_ks_block(&mut self, dest_ptr: *mut u8) {
            let mut block_ptr = dest_ptr as *mut u32;
            let res = run_rounds::<R>(&self.0.state);
            self.0.state[12] = self.0.state[12].wrapping_add(1);

            for val in res.iter() {
                block_ptr.write_unaligned(val.to_le());
                block_ptr = block_ptr.add(1);
            }
    }
    /// Generates 4 blocks and blindly writes it to the dest_ptr.
    #[inline(always)]
    #[cfg(feature = "rand_core")]
    pub(crate) unsafe fn rng_gen_ks_blocks(&mut self, mut dest_ptr: *mut u8) {
        for _i in 0..4 {
            self.write_ks_block(dest_ptr);
            dest_ptr = dest_ptr.add(64);
        }
    }
}

#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> StreamBackend for Backend<'a, R, V> {
    #[inline(always)]
    /// Writes a single block to `block`
    fn gen_ks_block(&mut self, block: &mut Block) {
        unsafe { 
            self.write_ks_block(block.as_mut_ptr()); 
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
