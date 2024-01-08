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
//! # #[cfg(feature = "cipher")]
//! {
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
//! }
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

pub(crate) mod backends;
#[cfg(feature = "cipher")]
mod chacha;
#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "rng")]
mod rng;
#[cfg(feature = "xchacha")]
mod xchacha;

mod variants;

use backends::ChaChaCore;

use variants::Variant;

#[cfg(feature = "cipher")]
pub use chacha::{ChaCha12, ChaCha20, ChaCha8, Key, KeyIvInit};
#[cfg(feature = "rng")]
pub use rand_core;
#[cfg(feature = "rng")]
pub use rng::{ChaCha12Core, ChaCha12Rng, ChaCha20Core, ChaCha20Rng, ChaCha8Core, ChaCha8Rng};

#[cfg(feature = "legacy")]
pub use legacy::{ChaCha20Legacy, LegacyNonce};
#[cfg(feature = "xchacha")]
pub use xchacha::{hchacha, XChaCha12, XChaCha20, XChaCha8, XNonce};

/// State initialization constant ("expand 32-byte k")
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];

/// Number of 32-bit words in the ChaCha state
const STATE_WORDS: usize = 16;

/// Marker type for a number of ChaCha rounds to perform.
pub trait Rounds: Copy {
    /// The amount of rounds to perform
    const COUNT: usize;
}

/// 8-rounds
#[derive(Copy, Clone)]
pub struct R8;

impl Rounds for R8 {
    const COUNT: usize = 4;
}

/// 12-rounds
#[derive(Copy, Clone)]
pub struct R12;

impl Rounds for R12 {
    const COUNT: usize = 6;
}

/// 20-rounds
#[derive(Copy, Clone)]
pub struct R20;

impl Rounds for R20 {
    const COUNT: usize = 10;
}