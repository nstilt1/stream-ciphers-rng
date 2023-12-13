//! Legacy version of ChaCha20 with a 64-bit nonce

use crate::chacha::Key;
use crate::{ChaChaCore, rounds::R20};
use cipher::{
    consts::{U32, U8},
    generic_array::GenericArray,
    IvSizeUser, KeyIvInit, KeySizeUser, StreamCipherCoreWrapper,
};

/// Nonce type used by [`ChaCha20Legacy`].
pub type LegacyNonce = GenericArray<u8, U8>;

use crate::variants::Legacy;

/// The ChaCha20 stream cipher (legacy "djb" construction with 64-bit nonce).
///
/// **WARNING:** this implementation uses 32-bit counter, while the original
/// implementation uses 64-bit counter. In other words, it does
/// not allow encrypting of more than 256 GiB of data.
pub type ChaCha20Legacy = StreamCipherCoreWrapper<ChaChaCore<R20, Legacy>>;

impl KeySizeUser for ChaChaCore<R20, Legacy> {
    type KeySize = U32;
}

impl IvSizeUser for ChaChaCore<R20, Legacy> {
    type IvSize = U8;
}

impl KeyIvInit for ChaChaCore<R20, Legacy> {
    #[inline(always)]
    fn new(key: &Key, iv: &LegacyNonce) -> Self {
        ChaChaCore::<R20, Legacy>::new(key.as_ref(), iv.as_ref())
    }
}
