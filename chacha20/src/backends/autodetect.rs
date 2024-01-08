//! Autodetection support for AVX2 CPU and SSE2 intrinsics on x86 CPUs, with
//! fallback to a portable version when they're unavailable.

use super::{avx2, soft, sse2};
use crate::{Rounds, STATE_WORDS, CONSTANTS, variants::Variant, backends::BackendType, StreamCipherSeekCore};
use core::mem::ManuallyDrop;
use cfg_if::cfg_if;

cpufeatures::new!(avx2_cpuid, "avx2");
cpufeatures::new!(sse2_cpuid, "sse2");

/// The ChaCha core function.
pub struct ChaChaCore<R: Rounds, V: Variant> {
    pub(crate) state: [u32; STATE_WORDS],
    inner: Inner<R, V>,
    avx2_token: avx2_cpuid::InitToken,
    sse2_token: sse2_cpuid::InitToken,
}

union Inner<R: Rounds, V: Variant> {
    avx2: ManuallyDrop<avx2::Backend<R, V>>,
    sse2: ManuallyDrop<sse2::Backend<R, V>>,
    soft: ManuallyDrop<soft::Backend<R, V>>,
}

#[cfg(feature = "cipher")]
use cipher::{consts::U64, StreamCipherCore};


impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
    /// Initialize ChaCha core function with the given key size, IV, and
    /// number of rounds.
    ///
    /// Attempts to use AVX2 if present, followed by SSE2, with fallback to a
    /// portable software implementation if neither are available.
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
        let (avx2_token, avx2_present) = avx2_cpuid::init_get();
        let (sse2_token, sse2_present) = sse2_cpuid::init_get();

        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                let inner = Inner {
                    soft: ManuallyDrop::new(soft::Backend::new(&state)),
                };
            } else if #[cfg(chacha20_force_avx2)] {
                let inner = Inner {
                    avx2: ManuallyDrop::new(avx2::Backend::new(&state)),
                };
            } else if #[cfg(chacha20_force_sse2)] {
                let inner = Inner {
                    sse2: ManuallyDrop::new(sse2::Backend::new(&state)),
                };
            } else {
                let inner = if avx2_present {
                    Inner {
                        avx2: ManuallyDrop::new(avx2::Backend::new(&state)),
                    }
                } else if sse2_present {
                    Inner {
                        sse2: ManuallyDrop::new(sse2::Backend::new(&state)),
                    }
                } else {
                    Inner {
                        soft: ManuallyDrop::new(soft::Backend::new(&state)),
                    }
                };
            }
        };

        Self {
            state,
            inner,
            avx2_token,
            sse2_token,
        }
    }

    #[inline]
    pub(crate) fn update_state(&mut self) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).update_state(&self.state) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).update_state(&self.state) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).update_state(&self.state) }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).update_state(&self.state) }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).update_state(&self.state) }
                } else {
                    unsafe { (*self.inner.soft).update_state(&self.state) }
                }
            }
        }
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
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).rng_inner(dest_ptr, num_blocks) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).rng_inner(dest_ptr, num_blocks) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).rng_inner(dest_ptr, num_blocks) }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).rng_inner(dest_ptr, num_blocks) }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).rng_inner(dest_ptr, num_blocks) }
                } else {
                    unsafe { (*self.inner.soft).rng_inner(dest_ptr, num_blocks) }
                }
            }
        }
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
impl<'a, R: Rounds, V: Variant> StreamCipherCore for ChaChaCore<R, V> {
    #[inline(always)]
    fn remaining_blocks(&self) -> Option<usize> {
        let rem = u32::MAX - self.get_block_pos();
        rem.try_into().ok()
    }

    /// Generate output, overwriting data already in the buffer.
    #[inline]
    fn process_with_backend(&mut self, f: impl cipher::StreamClosure<BlockSize = U64>) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).inner(&mut self.state[12], f) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).inner(&mut self.state[12], f) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).inner(&mut self.state[12], f) }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).inner(&mut self.state[12], f) }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).inner(&mut self.state[12], f) }
                } else {
                    unsafe { (*self.inner.soft).inner(&mut self.state[12], f) }
                }
            }
        }
    }
}

impl<'a, R: Rounds, V: Variant> Clone for ChaChaCore<R, V> {
    fn clone(&self) -> Self {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                let inner = Inner {
                    soft: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
                };
            } else if #[cfg(chacha20_force_avx2)] {
                let inner = Inner {
                    avx2: ManuallyDrop::new(unsafe { (*self.inner.avx2).clone() }),
                };
            } else if #[cfg(chacha20_force_sse2)] {
                let inner = Inner {
                    sse2: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
                };
            } else {
                let inner = if self.avx2_token.get() {
                    Inner {
                        avx2: ManuallyDrop::new(unsafe { (*self.inner.avx2).clone() }),
                    }
                } else if self.sse2_token.get() {
                    Inner {
                        sse2: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
                    }
                } else {
                    Inner {
                        soft: ManuallyDrop::new(unsafe { (*self.inner.soft).clone() }),
                    }
                };
            }
        };

        Self {
            state: self.state,
            inner,
            avx2_token: self.avx2_token,
            sse2_token: self.sse2_token,
        }
    }
}