//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use core::marker::PhantomData;

use crate::{variants::Variant, Rounds, STATE_WORDS, CONSTANTS};

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U1, U64},
    BlockSizeUser, ParBlocksSizeUser, StreamBackend, StreamClosure
};

use super::BackendType;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(chacha20_force_soft, not(any(target_arch = "x86", target_arch = "x86_64", all(target_arch = "aarch64", target_feature = "neon")))))] {
        #[cfg(feature = "cipher")]
        use cipher::{StreamCipherCore, StreamCipherSeekCore};

        #[derive(Clone)]
        pub struct ChaChaCore<R: Rounds, V: Variant> {
            pub(crate) state: [u32; STATE_WORDS], 
            backend: Backend<R, V>
        }

        impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
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
            fn update_state(&mut self) {
                self.backend.update_state(&self.state)
            }

            #[inline]
            pub(crate) fn get_block_pos_inner(&self) -> u32 {
                self.state[12]
            }

            #[inline]
            pub(crate) fn set_block_pos_inner(&mut self, pos: u32) {
                self.state[12] = pos;
                self.update_state();
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn set_nonce(&mut self, nonce: [u32; 3]) {
                self.state[13..16].copy_from_slice(&nonce);
                self.update_state();
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn get_nonce(&self) -> [u32; 3] {
                let mut result = [0u32; 3];
                result.copy_from_slice(&self.state[13..16]);
                result
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn get_seed(&self) -> [u32; 8] {
                let mut result = [0u32; 8];
                result.copy_from_slice(&self.state[4..12]);
                result
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn rng_inner(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
                self.backend.rng_inner(dest_ptr, num_blocks);
                self.state[12] = self.state[12].wrapping_add(num_blocks as u32);
            }
        }

        #[cfg(feature = "cipher")]
        impl<R: Rounds, V: Variant> StreamCipherSeekCore for ChaChaCore<R, V> {
            type Counter = u32;

            #[inline(always)]
            fn get_block_pos(&self) -> Self::Counter {
                self.state[12]
            }

            #[inline(always)]
            fn set_block_pos(&mut self, pos: Self::Counter) {
                self.state[12] = pos;
                self.update_state();
            }
        }

        #[cfg(feature = "cipher")]
        impl<R: Rounds, V: Variant> StreamCipherCore for ChaChaCore<R, V> {
            #[inline(always)]
            fn remaining_blocks(&self) -> Option<usize> {
                let rem = u32::MAX - self.get_block_pos();
                rem.try_into().ok()
            }

            /// Generate output, overwriting data already in the buffer.
            #[inline]
            fn process_with_backend(&mut self, f: impl cipher::StreamClosure<BlockSize = U64>) {
                self.backend.inner(&mut self.state[12], f)
            }
        }
    }
}

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

    fn new(state: &[u32; STATE_WORDS]) -> Self {
        Self {
            state: *state,
            _r: PhantomData,
            _variant: PhantomData
        }
    }

    #[inline]
    fn update_state(&mut self, state: &[u32]) {
        self.state[12..16].copy_from_slice(&state[12..16])
    }

    #[inline(always)]
    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    ///
    /// # Safety
    /// `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could cause a segmentation fault and/or undesired
    /// behavior.
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

    fn increment_counter(&mut self, amount: i32) {
        self.state[12] = self.state[12].wrapping_add(amount as u32);
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

#[cfg(feature = "cipher")]
impl<'a, R: Rounds, V: Variant> Backend<R, V> {
    #[inline]
    #[cfg(feature = "cipher")]
    pub(crate) fn inner<F>(&mut self, state_counter: &mut u32, f: F) 
    where
        R: Rounds,
        F: StreamClosure<BlockSize = U64>,
        V: Variant
    {
        f.call(self);
        *state_counter = self.state[12]
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
