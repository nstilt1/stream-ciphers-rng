//! Autodetection support for AVX2 CPU and SSE2 intrinsics on x86 CPUs, with
//! fallback to a portable version when they're unavailable.

use super::{avx2, soft, sse2};
use crate::{Rounds, STATE_WORDS, variants::Variant, backends::BackendType};
use core::mem::ManuallyDrop;
use cfg_if::cfg_if;

cpufeatures::new!(avx2_cpuid, "avx2");
cpufeatures::new!(sse2_cpuid, "sse2");

/// The ChaCha20 core function.
pub struct ChaChaCore<R: Rounds, V: Variant> {
    pub(crate) inner: Inner<R, V>,
    avx2_token: avx2_cpuid::InitToken,
    sse2_token: sse2_cpuid::InitToken,
}

pub(crate) union Inner<R: Rounds, V: Variant> {
    pub(crate) avx2: ManuallyDrop<avx2::Backend<R, V>>,
    pub(crate) sse2: ManuallyDrop<sse2::Backend<R, V>>,
    pub(crate) soft: ManuallyDrop<soft::Backend<R, V>>,
}

#[cfg(feature = "cipher")]
use cipher::{BlockSizeUser, consts::U64, StreamCipherCore};

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for ChaChaCore<R, V> {
    type BlockSize = U64;
}

impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
    /// Initialize ChaCha core function with the given key size, IV, and
    /// number of rounds.
    ///
    /// Attempts to use AVX2 if present, followed by SSE2, with fallback to a
    /// portable software implementation if neither are available.
    #[inline]
    pub(crate) fn new(key: &[u8; 32], iv: &[u8]) -> Self {
        let mut state = [0u32; STATE_WORDS];
        state[0..4].copy_from_slice(&crate::CONSTANTS);
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
                Inner {
                    soft: ManuallyDrop::new(soft::Backend::new(&mut state)),
                }
            } else if #[cfg(chacha20_force_avx2)] {
                Inner {
                    avx2: ManuallyDrop::new(avx2::Backend::new(&mut state)),
                }
            } else if #[cfg(chacha20_force_sse2)] {
                Inner {
                    sse2: ManuallyDrop::new(sse2::Backend::new(&mut state)),
                }
            } else {
                let inner = if avx2_present {
                    Inner {
                        avx2: ManuallyDrop::new(avx2::Backend::new(&mut state)),
                    }
                } else if sse2_present {
                    Inner {
                        sse2: ManuallyDrop::new(sse2::Backend::new(&mut state)),
                    }
                } else {
                    Inner {
                        soft: ManuallyDrop::new(soft::Backend::new(&mut state)),
                    }
                };
            }
        };

        Self {
            inner,
            avx2_token,
            sse2_token,
        }
    }

    #[inline]
    pub(crate) fn get_block_pos(&self) -> u32 {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { self.inner.soft.get_block_pos() }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { self.inner.avx2.get_block_pos() }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { self.inner.sse2.get_block_pos() }
            } else {
                if self.avx2_token.get() {
                    unsafe { self.inner.avx2.get_block_pos() }
                } else if self.sse2_token.get() {
                    unsafe { self.inner.sse2.get_block_pos() }
                } else {
                    unsafe { self.inner.soft.get_block_pos() }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn set_block_pos(&mut self, pos: u32) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).set_block_pos(pos) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).set_block_pos(pos) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).set_block_pos(pos) }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).set_block_pos(pos) }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).set_block_pos(pos) }
                } else {
                    unsafe { (*self.inner.soft).set_block_pos(pos) }
                }
            }
        }
    }

    #[inline]
    #[cfg(feature = "rand_core")]
    pub(crate) fn set_nonce(&mut self, nonce: [u32; 3]) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).set_nonce(nonce) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).set_nonce(nonce) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).set_nonce(nonce) }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).set_nonce(nonce) }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).set_nonce(nonce) }
                } else {
                    unsafe { (*self.inner.soft).set_nonce(nonce) }
                }
            }
        }
    }

    #[inline]
    #[cfg(feature = "rand_core")]
    pub(crate) fn get_nonce(&self) -> [u32; 3] {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).get_nonce() }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).get_nonce() }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).get_nonce() }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).get_nonce() }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).get_nonce() }
                } else {
                    unsafe { (*self.inner.soft).get_nonce() }
                }
            }
        }
    }

    #[inline]
    #[cfg(feature = "rand_core")]
    pub(crate) fn get_seed(&self) -> [u32; 8] {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { (*self.inner.soft).get_seed() }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { (*self.inner.avx2).get_seed() }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (*self.inner.sse2).get_seed() }
            } else {
                if self.avx2_token.get() {
                    unsafe { (*self.inner.avx2).get_seed() }
                } else if self.sse2_token.get() {
                    unsafe { (*self.inner.sse2).get_seed() }
                } else {
                    unsafe { (*self.inner.soft).get_seed() }
                }
            }
        }
    }

    #[inline]
    #[cfg(feature = "rand_core")]
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
    }
}

impl<R: Rounds, V: Variant> Drop for ChaChaCore<R, V> {
    fn drop(&mut self) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { ManuallyDrop::drop(&mut self.inner.soft) }
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { ManuallyDrop::drop(&mut self.inner.avx2) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { ManuallyDrop::drop(&mut self.inner.sse2) }
            } else {
                if self.avx2_token.get() {
                    unsafe { ManuallyDrop::drop(&mut self.inner.avx2) }
                } else if self.sse2_token.get() {
                    unsafe { ManuallyDrop::drop(&mut self.inner.sse2) }
                } else {
                    unsafe { ManuallyDrop::drop(&mut self.inner.soft) }
                }
            }
        }
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
    #[cfg(feature = "cipher")]
    fn process_with_backend(&mut self, f: impl cipher::StreamClosure<BlockSize = U64>) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                unsafe { f.call(&mut *self.inner.soft)}
            } else if #[cfg(chacha20_force_avx2)] {
                unsafe { f.call(&mut *self.inner.avx2) }
            } else if #[cfg(chacha20_force_sse2)] {
                unsafe { (&mut *self.inner.sse2) }
            } else {
                if self.avx2_token.get() {
                    unsafe { f.call(&mut *self.inner.avx2) }
                } else if self.sse2_token.get() {
                    unsafe { f.call(&mut *self.inner.sse2) }
                } else {
                    unsafe { f.call(&mut *self.inner.soft) }
                }
            }
        }
    }
}

impl<'a, R: Rounds, V: Variant> Clone for ChaChaCore<R, V> {
    fn clone(&self) -> Self {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                Inner {
                    soft: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
                }
            } else if #[cfg(chacha20_force_avx2)] {
                Inner {
                    avx2: ManuallyDrop::new(unsafe { (*self.inner.avx2).clone() }),
                }
            } else if #[cfg(chacha20_force_sse2)] {
                Inner {
                    sse2: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
                }
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
            inner,
            avx2_token: self.avx2_token,
            sse2_token: self.sse2_token,
        }
    }
}