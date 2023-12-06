pub use cipher::KeyIvInit;
use cipher::{
    consts::{U12, U32, U64},
    generic_array::GenericArray,
    IvSizeUser, KeySizeUser, StreamCipherCoreWrapper,
};

use crate::{ChaChaCore, Rounds, IETF, R12, R20, R8};

/// Key type used by all ChaCha variants.
pub type Key = GenericArray<u8, U32>;

/// Nonce type used by ChaCha variants.
pub type Nonce = GenericArray<u8, U12>;

/// ChaCha8 stream cipher (reduced-round variant of [`ChaCha20`] with 8 rounds)
pub type ChaCha8 = StreamCipherCoreWrapper<ChaChaCore<R8, IETF>>;

/// ChaCha12 stream cipher (reduced-round variant of [`ChaCha20`] with 12 rounds)
pub type ChaCha12 = StreamCipherCoreWrapper<ChaChaCore<R12, IETF>>;

/// ChaCha20 stream cipher (RFC 8439 version with 96-bit nonce)
pub type ChaCha20 = StreamCipherCoreWrapper<ChaChaCore<R20, IETF>>;

pub(crate) type Block = GenericArray<u8, U64>;

impl<R: Rounds> KeySizeUser for ChaChaCore<R, IETF> {
    type KeySize = U32;
}

impl<R: Rounds> IvSizeUser for ChaChaCore<R, IETF> {
    type IvSize = U12;
}
impl<R: Rounds> KeyIvInit for ChaChaCore<R, IETF> {
    #[inline]
    fn new(key: &Key, iv: &Nonce) -> Self {
        ChaChaCore::<R, IETF>::new(key.as_ref(), iv.as_ref())
    }
}
