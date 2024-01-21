//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use core::marker::PhantomData;

use crate::{variants::Variant, Rounds, STATE_WORDS};

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
        use crate::{CONSTANTS, impl_chacha_core};

        #[cfg(feature = "cipher")]
        use cipher::{StreamCipherCore, StreamCipherSeekCore};

        #[derive(Clone)]
        pub struct ChaChaCore<R: Rounds, V: Variant> {
            pub(crate) state: [u32; STATE_WORDS], 
            backend: Backend<R, V>
        }

        impl_chacha_core!();

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
            pub(crate) fn update_state(&mut self) {
                self.backend.update_state(&self.state)
            }

            /// Generates `num_blocks` blocks of output and writes them `dest_ptr`.
            ///
            /// # Safety
            /// - `dest_ptr` must have `num_blocks * 64 bytes` available to be overwritten.
            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) unsafe fn generate(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
                unsafe {
                    self.backend.write_ks_blocks(dest_ptr, num_blocks);
                }
                self.state[12] = self.state[12].wrapping_add(num_blocks as u32);
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
            fn process_with_backend(&mut self, f: impl StreamClosure<BlockSize = U64>) {
                f.call(&mut self.backend);
                self.state[12] = self.backend.state[12]
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct Backend<R: Rounds, V: Variant>{
    state: [u32; 16],
    results: [u32; 16],
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
            results: [0u32; 16],
            _r: PhantomData,
            _variant: PhantomData
        }
    }

    #[inline]
    fn update_state(&mut self, state: &[u32]) {
        self.state[12..16].copy_from_slice(&state[12..16])
    }

    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    ///
    /// # Safety
    /// - `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could cause a segmentation fault and/or undesired
    /// behavior.
    #[inline(always)]
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        let mut block_ptr = dest_ptr as *mut u32;
        for _i in 0..num_blocks {
            self.run_rounds();
            self.increment_counter(1);

            for val in self.results.iter() {
                block_ptr.write_unaligned(val.to_le());
                block_ptr = block_ptr.add(1);
            }
        }
    }

    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    ///
    /// # Safety
    /// - `dest_ptr` must have at least `64 * num_blocks` bytes available to be
    /// overwritten, or else it could cause a segmentation fault and/or undesired
    /// behavior.
    /// - `dest_ptr` should be aligned on a 16-byte boundary
    #[cfg(feature = "rng")]
    #[inline(always)]
    unsafe fn write_ks_blocks_aligned(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        let mut block_ptr = dest_ptr as *mut u32;
        for _i in 0..num_blocks {
            self.run_rounds();
            self.increment_counter(1);

            for val in self.results.iter() {
                block_ptr.write(val.to_le());
                block_ptr = block_ptr.add(1);
            }
        }
    }

    fn increment_counter(&mut self, amount: i32) {
        self.state[12] = self.state[12].wrapping_add(amount as u32);
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamBackend for Backend<R, V> {
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
impl<R: Rounds, V: Variant> Backend<R, V> {
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

impl<R: Rounds, V: Variant> Backend<R, V> {
    #[inline(always)]
    fn run_rounds(&mut self) {
        self.results = self.state;

        for _ in 0..R::COUNT {
            // column rounds
            quarter_round(0, 4, 8, 12, &mut self.results);
            quarter_round(1, 5, 9, 13, &mut self.results);
            quarter_round(2, 6, 10, 14, &mut self.results);
            quarter_round(3, 7, 11, 15, &mut self.results);

            // diagonal rounds
            quarter_round(0, 5, 10, 15, &mut self.results);
            quarter_round(1, 6, 11, 12, &mut self.results);
            quarter_round(2, 7, 8, 13, &mut self.results);
            quarter_round(3, 4, 9, 14, &mut self.results);
        }

        for (s1, s0) in self.results.iter_mut().zip(self.state.iter()) {
            *s1 = s1.wrapping_add(*s0);
        }
    }
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