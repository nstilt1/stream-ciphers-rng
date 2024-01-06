//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use crate::{variants::Variant, ChaChaCore, Rounds, STATE_WORDS};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::U1,
    ParBlocksSizeUser, StreamBackend,
};

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> ParBlocksSizeUser for ChaChaCore<R, V> {
    type ParBlocksSize = U1;
}

impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
    #[inline(always)]
    /// Generates a single keystream block and blindly writes it to `dest_ptr`
    ///
    /// # Safety
    /// `dest_ptr` must have at least 64 bytes available to be overwritten, or else it
    /// could cause a segmentation fault or undesired behavior
    pub(crate) unsafe fn write_ks_block(&mut self, dest_ptr: *mut u8) {
        let mut block_ptr = dest_ptr as *mut u32;
        let res = run_rounds::<R>(&self.state);
        self.state[12] = self.state[12].wrapping_add(1);

        for val in res.iter() {
            block_ptr.write_unaligned(val.to_le());
            block_ptr = block_ptr.add(1);
        }
    }
    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could cause a segmentation fault or undesired 
    /// behavior
    #[inline(always)]
    #[cfg(feature = "rng")]
    pub(crate) unsafe fn rng_gen_ks_blocks(&mut self, mut dest_ptr: *mut u8, num_blocks: usize) {
        for _i in 0..num_blocks {
            self.write_ks_block(dest_ptr);
            dest_ptr = dest_ptr.add(64);
        }
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamBackend for ChaChaCore<R, V> {
    #[inline(always)]
    /// Writes a single block to `block`
    fn gen_ks_block(&mut self, block: &mut Block) {
        // SAFETY: `Block` is a 64-byte array
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
