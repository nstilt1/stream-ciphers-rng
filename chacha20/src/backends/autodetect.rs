//! Autodetection support for AVX2 CPU and SSE2 intrinsics on x86 CPUs, with
//! fallback to a portable version when they're unavailable.

use super::{avx2, soft, sse2};
use crate::{ChaChaCore, Rounds, STATE_WORDS, variants::Variant, backends::BackendType};
use core::mem::ManuallyDrop;
use cfg_if::cfg_if;

cpufeatures::new!(avx2_cpuid, "avx2");
cpufeatures::new!(sse2_cpuid, "sse2");

/// The ChaCha20 core function.
pub struct Backend<R: Rounds, V: Variant> {
    inner: Inner<R, V>,
    avx2_token: avx2_cpuid::InitToken,
    sse2_token: sse2_cpuid::InitToken,
}

enum Inner<R: Rounds, V: Variant> {
    Avx2(ManuallyDrop<avx2::Backend<R, V>>),
    Sse2(ManuallyDrop<sse2::Backend<R, V>>),
    Soft(ManuallyDrop<soft::Backend<R, V>>),
}


#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamBackend for Inner<R, V> {
    fn gen_ks_block(&mut self, block: &mut cipher::Block<Self>) {
        match self {
            Self::Avx2(backend) => backend.gen_ks_block(block),
            Self::Soft(backend) => backend.gen_ks_block(block),
            Self::Sse2(backend) => backend.gen_ks_block(block)
        }
    }
}

impl<R: Rounds, V: Variant> BackendType for Inner<R, V> {
    fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                Self::Soft(ManuallyDrop::new(soft::Backend<R, V>::new(state)))
            } else if #[cfg(chacha20_force_avx2)] {
                Self::Avx2(ManuallyDrop::new(avx2::Backend<R, V>::new(state)))
            } else if #[cfg(chacha20_force_sse2)] {
                Self::Sse2(ManuallyDrop::new(sse2::Backend<R, V>::new(state)))
            } else {
                if avx2_cpuid::get() {
                    Self::Avx2(ManuallyDrop::new(avx2::Backend::new(state)))
                } else if sse2_cpuid::get() {
                    Self::Sse2(ManuallyDrop::new(sse2::Backend::new(state)))
                } else {
                    Self::Soft(ManuallyDrop::new(soft::Backend::new(state)))
                }
            }
        }
    }
    fn get_block_pos(&self) -> u32 {
        match self {
            Inner::Avx2(backend) => backend.get_block_pos(),
            Inner::Soft(backend) => backend.get_block_pos(),
            Inner::Sse2(backend) => backend.get_block_pos()
        }
    }
    fn set_block_pos(&mut self, pos: u32) {
        match self {
            Inner::Avx2(backend) => backend.set_block_pos(pos),
            Inner::Soft(backend) => backend.set_block_pos(pos),
            Inner::Sse2(backend) => backend.set_block_pos(pos)
        }
    }
    fn increment_counter(&mut self, amount: i32) {
        match self {
            Inner::Avx2(backend) => backend.increment_counter(amount),
            Inner::Soft(backend) => backend.increment_counter(amount),
            Inner::Sse2(backend) => backend.increment_counter(amount)
        }
    }
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        match self {
            Inner::Avx2(backend) => backend.write_ks_blocks(dest_ptr, num_blocks),
            Inner::Soft(backend) => backend.write_ks_blocks(dest_ptr, num_blocks),
            Inner::Sse2(backend) => backend.write_ks_blocks(dest_ptr, num_blocks)
        }
    }
}

use cipher::StreamBackend;
#[cfg(feature = "cipher")]
use cipher::{BlockSizeUser, consts::U64};

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for Backend<R, V> {
    type BlockSize = U64;
}

impl<R: Rounds, V: Variant> Backend<R, V> {
    /// Initialize ChaCha core function with the given key size, IV, and
    /// number of rounds.
    ///
    /// Attempts to use AVX2 if present, followed by SSE2, with fallback to a
    /// portable software implementation if neither are available.
    #[inline]
    pub fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        let (avx2_token, avx2_present) = avx2_cpuid::init_get();
        let (sse2_token, sse2_present) = sse2_cpuid::init_get();

        let inner = Inner::new(state);

        Self {
            inner,
            avx2_token,
            sse2_token,
        }
    }

    #[inline]
    pub(crate) fn get_block_pos(&self) -> u32 {
        self.inner.get_block_pos()
    }

    /// Generate output, overwriting data already in the buffer.
    #[inline]
    #[cfg(feature = "cipher")]
    pub fn process_with_backend(&mut self, f: impl cipher::StreamClosure<BlockSize = U64>) {
        f.call(&mut self.inner)
    }
}

impl<'a, R: Rounds, V: Variant> Clone for Backend<R, V> {
    fn clone(&self) -> Self {
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

        Self {
            inner,
            avx2_token: self.avx2_token,
            sse2_token: self.sse2_token,
        }
    }
}