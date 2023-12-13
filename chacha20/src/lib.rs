//! Implementation of the [ChaCha] family of stream ciphers.
//!
//! Cipher functionality is accessed using traits from re-exported [`cipher`] crate.
//!
//! ChaCha stream ciphers are lightweight and amenable to fast, constant-time
//! implementations in software. It improves upon the previous [Salsa] design,
//! providing increased per-round diffusion with no cost to performance.
//!
//! This crate contains the following variants of the ChaCha20 core algorithm:
//!
//! - [`ChaCha20`]: standard IETF variant with 96-bit nonce
//! - [`ChaCha8`] / [`ChaCha12`]: reduced round variants of ChaCha20
//! - [`XChaCha20`]: 192-bit extended nonce variant
//! - [`XChaCha8`] / [`XChaCha12`]: reduced round variants of XChaCha20
//! - [`ChaCha20Legacy`]: "djb" variant with 64-bit nonce.
//! **WARNING:** This implementation internally uses 32-bit counter,
//! while the original implementation uses 64-bit counter. In other words,
//! it does not allow encryption of more than 256 GiB of data.
//!
//! # ⚠️ Security Warning: Hazmat!
//!
//! This crate does not ensure ciphertexts are authentic, which can lead to
//! serious vulnerabilities if used incorrectly!
//!
//! If in doubt, use the [`chacha20poly1305`] crate instead, which provides
//! an authenticated mode on top of ChaCha20.
//!
//! **USE AT YOUR OWN RISK!**
//!
//! # Diagram
//!
//! This diagram illustrates the ChaCha quarter round function.
//! Each round consists of four quarter-rounds:
//!
//! <img src="https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/img/stream-ciphers/chacha20.png" width="300px">
//!
//! Legend:
//!
//! - ⊞ add
//! - ‹‹‹ rotate
//! - ⊕ xor
//!
//! # Example
//! ```
//! use chacha20::ChaCha20;
//! // Import relevant traits
//! use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
//! use hex_literal::hex;
//!
//! let key = [0x42; 32];
//! let nonce = [0x24; 12];
//! let plaintext = hex!("00010203 04050607 08090A0B 0C0D0E0F");
//! let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");
//!
//! // Key and IV must be references to the `GenericArray` type.
//! // Here we use the `Into` trait to convert arrays into it.
//! let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
//!
//! let mut buffer = plaintext.clone();
//!
//! // apply keystream (encrypt)
//! cipher.apply_keystream(&mut buffer);
//! assert_eq!(buffer, ciphertext);
//!
//! let ciphertext = buffer.clone();
//!
//! // ChaCha ciphers support seeking
//! cipher.seek(0u32);
//!
//! // decrypt ciphertext by applying keystream again
//! cipher.apply_keystream(&mut buffer);
//! assert_eq!(buffer, plaintext);
//!
//! // stream ciphers can be used with streaming messages
//! cipher.seek(0u32);
//! for chunk in buffer.chunks_mut(3) {
//!     cipher.apply_keystream(chunk);
//! }
//! assert_eq!(buffer, ciphertext);
//! ```
//!
//! # Configuration Flags
//!
//! You can modify crate using the following configuration flags:
//!
//! - `chacha20_force_avx2`: force AVX2 backend on x86/x86_64 targets.
//!   Requires enabled AVX2 target feature. Ignored on non-x86(-64) targets.
//! - `chacha20_force_neon`: force NEON backend on ARM targets.
//!   Requires enabled NEON target feature. Ignored on non-ARM targets. Nightly-only.
//! - `chacha20_force_soft`: force software backend.
//! - `chacha20_force_sse2`: force SSE2 backend on x86/x86_64 targets.
//!   Requires enabled SSE2 target feature. Ignored on non-x86(-64) targets.
//!
//! The flags can be enabled using `RUSTFLAGS` environmental variable
//! (e.g. `RUSTFLAGS="--cfg chacha20_force_avx2"`) or by modifying `.cargo/config`.
//!
//! You SHOULD NOT enable several `force` flags simultaneously.
//!
//! [ChaCha]: https://tools.ietf.org/html/rfc8439
//! [Salsa]: https://en.wikipedia.org/wiki/Salsa20
//! [`chacha20poly1305`]: https://docs.rs/chacha20poly1305

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"
)]
#![allow(clippy::needless_range_loop)]
#![warn(missing_docs, rust_2018_idioms, trivial_casts, unused_qualifications)]

#[cfg(feature = "cipher")]
pub use cipher;
#[cfg(feature = "cipher")]
use cipher::{consts::U64, BlockSizeUser, StreamCipherCore, StreamCipherSeekCore};

use cfg_if::cfg_if;
use core::marker::PhantomData;

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

mod backends;
#[cfg(feature = "cipher")]
mod chacha;
#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "rand_core")]
mod rng;
#[cfg(feature = "xchacha")]
mod xchacha;

mod variants;
use variants::{Ietf, Variant};

#[cfg(feature = "cipher")]
pub use chacha::{ChaCha12, ChaCha20, ChaCha8, Key, KeyIvInit};
#[cfg(feature = "rand_core")]
pub use rand_core;
#[cfg(feature = "rand_core")]
pub use rng::{ChaCha12Core, ChaCha12Rng, ChaCha20Core, ChaCha20Rng, ChaCha8Core, ChaCha8Rng};

#[cfg(feature = "legacy")]
pub use legacy::{ChaCha20Legacy, LegacyNonce};
#[cfg(feature = "xchacha")]
pub use xchacha::{hchacha, XChaCha12, XChaCha20, XChaCha8, XNonce};

/// State initialization constant ("expand 32-byte k")
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];

/// Number of 32-bit words in the ChaCha state
const STATE_WORDS: usize = 16;

mod rounds;
use rounds::Rounds;

cfg_if! {
    if #[cfg(chacha20_force_soft)] {
        type Tokens = ();
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        cfg_if! {
            if #[cfg(chacha20_force_avx2)] {
                #[cfg(not(target_feature = "avx2"))]
                compile_error!("You must enable `avx2` target feature with \
                    `chacha20_force_avx2` configuration option");
                type Tokens = ();
            } else if #[cfg(chacha20_force_sse2)] {
                #[cfg(not(target_feature = "sse2"))]
                compile_error!("You must enable `sse2` target feature with \
                    `chacha20_force_sse2` configuration option");
                type Tokens = ();
            } else {
                cpufeatures::new!(avx2_cpuid, "avx2");
                cpufeatures::new!(sse2_cpuid, "sse2");
                type Tokens = (avx2_cpuid::InitToken, sse2_cpuid::InitToken);
            }
        }
    } else {
        type Tokens = ();
    }
}

/// The ChaCha core function.
#[cfg_attr(feature = "rand_core", derive(Clone))]
pub struct ChaChaCore<R: Rounds, V: Variant> {
    /// Internal state of the core function
    state: [u32; STATE_WORDS],
    /// CPU target feature tokens
    #[allow(dead_code)]
    tokens: Tokens,
    /// Number of rounds to perform
    rounds: PhantomData<R>,
    /// the variant of the implementation
    variant: PhantomData<V>,
}

impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
    /// Constructs a ChaChaCore with the specified key, iv, and amount of rounds.
    /// You must ensure that the iv is of the correct size when using this method
    /// directly.
    fn new(key: &[u8; 32], iv: &[u8]) -> Self {
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

        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                let tokens = ();
            } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                cfg_if! {
                    if #[cfg(chacha20_force_avx2)] {
                        let tokens = ();
                    } else if #[cfg(chacha20_force_sse2)] {
                        let tokens = ();
                    } else {
                        let tokens = (avx2_cpuid::init(), sse2_cpuid::init());
                    }
                }
            } else {
                let tokens = ();
            }
        }
        Self {
            state,
            tokens,
            rounds: PhantomData,
            variant: PhantomData,
        }
    }

    /// Generates 4 blocks in parallel with avx2 & neon, but merely fills
    /// 4 blocks with sse2 & soft, writing them to the pointer's address.
    #[cfg(feature = "rand_core")]
    fn generate(&mut self, dest_ptr: *mut u8) {
        assert!(!dest_ptr.is_null());
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                backends::soft::Backend(self).rng_gen_ks_blocks(dest_ptr);
            } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                cfg_if! {
                    if #[cfg(chacha20_force_avx2)] {
                        unsafe {
                            backends::avx2::rng_inner::<R, V>(self, dest_ptr);
                        }
                    } else if #[cfg(chacha20_force_sse2)] {
                        unsafe {
                            backends::sse2::rng_inner::<R, V>(self, dest_ptr);
                        }
                    } else {
                        let (avx2_token, sse2_token) = self.tokens;
                        if avx2_token.get() {
                            unsafe {
                                backends::avx2::rng_inner::<R, V>(self, dest_ptr);
                            }
                        } else if sse2_token.get() {
                            unsafe {
                                backends::sse2::rng_inner::<R, V>(self, dest_ptr);
                            }
                        } else {
                            backends::soft::Backend(self).rng_gen_ks_blocks(dest_ptr);
                        }
                    }
                }
            } else if #[cfg(all(chacha20_force_neon, target_arch = "aarch64", target_feature = "neon"))] {
                unsafe {
                    backends::neon::rng_inner::<R, V>(self, dest_ptr);
                }
            } else {
                backends::soft::Backend(self).rng_gen_ks_blocks(dest_ptr);
            }
        }
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamCipherSeekCore for ChaChaCore<R, V> {
    type Counter = V::Counter;

    #[inline(always)]
    fn get_block_pos(&self) -> Self::Counter {
        // seems like we have to cast a spell to do anything with this type
        V::get_pos_helper(&self.state[12..=13])
    }

    #[inline(always)]
    fn set_block_pos(&mut self, pos: Self::Counter) {
        // seems like we have to cast a spell to do anything with this type
        let result = V::set_pos_helper(pos);
        self.state[12] = result[0];
        if !V::IS_32_BIT_COUNTER {
            self.state[13] = result[1];
        }
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamCipherCore for ChaChaCore<R, V> {
    #[inline(always)]
    fn remaining_blocks(&self) -> Option<usize> {
        V::remaining_blocks(self.get_block_pos())
    }

    fn process_with_backend(&mut self, f: impl cipher::StreamClosure<BlockSize = Self::BlockSize>) {
        cfg_if! {
            if #[cfg(chacha20_force_soft)] {
                f.call(&mut backends::soft::Backend(self));
            } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                cfg_if! {
                    if #[cfg(chacha20_force_avx2)] {
                        unsafe {
                            backends::avx2::inner::<R, _, V>(&mut self.state, f);
                        }
                    } else if #[cfg(chacha20_force_sse2)] {
                        unsafe {
                            backends::sse2::inner::<R, _, V>(&mut self.state, f);
                        }
                    } else {
                        let (avx2_token, sse2_token) = self.tokens;
                        if avx2_token.get() {
                            unsafe {
                                backends::avx2::inner::<R, _, V>(&mut self.state, f);
                            }
                        } else if sse2_token.get() {
                            unsafe {
                                backends::sse2::inner::<R, _, V>(&mut self.state, f);
                            }
                        } else {
                            f.call(&mut backends::soft::Backend(self));
                        }
                    }
                }
            } else if #[cfg(all(chacha20_force_neon, target_arch = "aarch64", target_feature = "neon"))] {
                unsafe {
                    backends::neon::inner::<R, _, V>(&mut self.state, f);
                }
            } else {
                f.call(&mut backends::soft::Backend(self));
            }
        }
    }
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for ChaChaCore<R, V> {
    type BlockSize = U64;
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
