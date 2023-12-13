//! XChaCha is an extended nonce variant of ChaCha

use cipher::{
    consts::{U16, U24, U32},
    generic_array::GenericArray,
    IvSizeUser, KeyIvInit, KeySizeUser, StreamCipherCoreWrapper,
};

use crate::{variants::XChaChaVariant, ChaChaCore, Rounds, CONSTANTS, R12, R20, R8, STATE_WORDS};

/// Key type used by all ChaCha variants.
pub type Key = GenericArray<u8, U32>;

/// Nonce type used by XChaCha variants.
pub type XNonce = GenericArray<u8, U24>;

/// XChaCha is a ChaCha20 variant with an extended 192-bit (24-byte) nonce.
///
/// The construction is an adaptation of the same techniques used by
/// XChaCha as described in the paper "Extending the Salsa20 Nonce",
/// applied to the 96-bit nonce variant of ChaCha20, and derive a
/// separate subkey/nonce for each extended nonce:
///
/// <https://cr.yp.to/snuffle/xsalsa-20081128.pdf>
///
/// No authoritative specification exists for XChaCha20, however the
/// construction has "rough consensus and running code" in the form of
/// several interoperable libraries and protocols (e.g. libsodium, WireGuard)
/// and is documented in an (expired) IETF draft:
///
/// <https://tools.ietf.org/html/draft-arciszewski-xchacha-03>
pub type XChaCha20 = StreamCipherCoreWrapper<ChaChaCore<R20, XChaChaVariant>>;
/// XChaCha12 stream cipher (reduced-round variant of [`XChaCha20`] with 12 rounds)
pub type XChaCha12 = StreamCipherCoreWrapper<ChaChaCore<R12, XChaChaVariant>>;
/// XChaCha8 stream cipher (reduced-round variant of [`XChaCha20`] with 8 rounds)
pub type XChaCha8 = StreamCipherCoreWrapper<ChaChaCore<R8, XChaChaVariant>>;

impl<R: Rounds> KeySizeUser for ChaChaCore<R, XChaChaVariant> {
    type KeySize = U32;
}

impl<R: Rounds> IvSizeUser for ChaChaCore<R, XChaChaVariant> {
    type IvSize = U24;
}

impl<R: Rounds> KeyIvInit for ChaChaCore<R, XChaChaVariant> {
    fn new(key: &Key, iv: &XNonce) -> Self {
        let subkey = hchacha::<R>(key, iv[..16].as_ref().into());

        let mut nonce = [0u8; 12];
        // first 4 bytes are 0, last 8 bytes are last 8 from the iv
        // according to draft-arciszewski-xchacha-03
        nonce[4..].copy_from_slice(&iv[16..]);
        ChaChaCore::<R, XChaChaVariant>::new(subkey.as_ref(), &nonce)
    }
}

/// The HChaCha function: adapts the ChaCha core function in the same
/// manner that HSalsa adapts the Salsa function.
///
/// HChaCha takes 512-bits of input:
///
/// - Constants: `u32` x 4
/// - Key: `u32` x 8
/// - Nonce: `u32` x 4
///
/// It produces 256-bits of output suitable for use as a ChaCha key
///
/// For more information on HSalsa on which HChaCha is based, see:
///
/// <http://cr.yp.to/snuffle/xsalsa-20110204.pdf>
pub fn hchacha<R: Rounds>(key: &Key, input: &GenericArray<u8, U16>) -> GenericArray<u8, U32> {
    let mut state = [0u32; STATE_WORDS];
    state[..4].copy_from_slice(&CONSTANTS);

    let key_chunks = key.chunks_exact(4);
    for (v, chunk) in state[4..12].iter_mut().zip(key_chunks) {
        *v = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    let input_chunks = input.chunks_exact(4);
    for (v, chunk) in state[12..16].iter_mut().zip(input_chunks) {
        *v = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    // R rounds consisting of R/2 column rounds and R/2 diagonal rounds
    for _ in 0..R::COUNT {
        // column rounds
        quarter_round(0, 4, 8, 12, &mut state);
        quarter_round(1, 5, 9, 13, &mut state);
        quarter_round(2, 6, 10, 14, &mut state);
        quarter_round(3, 7, 11, 15, &mut state);

        // diagonal rounds
        quarter_round(0, 5, 10, 15, &mut state);
        quarter_round(1, 6, 11, 12, &mut state);
        quarter_round(2, 7, 8, 13, &mut state);
        quarter_round(3, 4, 9, 14, &mut state);
    }

    let mut output = GenericArray::default();

    for (chunk, val) in output[..16].chunks_exact_mut(4).zip(&state[..4]) {
        chunk.copy_from_slice(&val.to_le_bytes());
    }

    for (chunk, val) in output[16..].chunks_exact_mut(4).zip(&state[12..]) {
        chunk.copy_from_slice(&val.to_le_bytes());
    }

    output
}

/// The ChaCha20 quarter round function
// for simplicity this function is copied from the software backend
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

#[cfg(test)]
mod hchacha20_tests {
    use crate::R20;

    use super::*;
    use hex_literal::hex;

    /// Test vectors from:
    /// https://tools.ietf.org/id/draft-arciszewski-xchacha-03.html#rfc.section.2.2.1
    #[test]
    fn test_vector() {
        const KEY: [u8; 32] = hex!(
            "000102030405060708090a0b0c0d0e0f"
            "101112131415161718191a1b1c1d1e1f"
        );

        const INPUT: [u8; 16] = hex!("000000090000004a0000000031415927");

        const OUTPUT: [u8; 32] = hex!(
            "82413b4227b27bfed30e42508a877d73"
            "a0f9e4d58a74a853c12ec41326d3ecdc"
        );

        let actual = hchacha::<R20>(
            GenericArray::from_slice(&KEY),
            GenericArray::from_slice(&INPUT),
        );
        assert_eq!(actual.as_slice(), &OUTPUT);
    }
}
