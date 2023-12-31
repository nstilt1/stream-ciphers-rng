use crate::{Rounds, STATE_WORDS, Variant};
use core::marker::PhantomData;

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(feature = "cipher")]
use cipher::{
    BlockSizeUser, ParBlocksSizeUser, StreamBackend,
    consts::{U1, U64}
};

use super::BackendType;

/// An i32 of all 1s. I'm not sure if i32::MAX would work, since a bit might be 0
const I32_MAX: i32 = 0xFFFF_FFFFu32 as i32;

#[derive(Clone)]
pub(crate) struct Backend<R: Rounds, V: Variant> {
    v: [__m128i; 4],
    _pd: PhantomData<R>,
    _variant: PhantomData<V>
}

impl<R: Rounds, V: Variant> BackendType for Backend<R, V> {
    const PAR_BLOCKS: usize = 1;
    #[inline]
    fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            Self {
                v: [
                    _mm_loadu_si128(state_ptr.add(0)),
                    _mm_loadu_si128(state_ptr.add(1)),
                    _mm_loadu_si128(state_ptr.add(2)),
                    _mm_loadu_si128(state_ptr.add(3)),
                ],
                _pd: PhantomData,
                _variant: PhantomData
            }
        }
    }

    fn get_block_pos(&self) -> u32 {
        unsafe { _mm_cvtsi128_si32(self.v[3]) as u32 }
    }

    fn set_block_pos(&mut self, pos: u32) {
        unsafe {
            let mask = _mm_set_epi32(0, 0, 0, I32_MAX);
            self.v[3] = _mm_andnot_si128(mask, self.v[3]);
            self.v[3] = _mm_add_epi32(self.v[3], _mm_set_epi32(0, 0, 0, pos as i32));
        }
    }

    #[cfg(feature = "rand_core")]
    fn set_nonce(&mut self, nonce: [u32; 3]) {
        unsafe {
            let mask = _mm_set_epi32(0, 0, 0, I32_MAX);
            self.v[3] = _mm_and_si128(mask, self.v[3]);
            self.v[3] = _mm_add_epi32(_mm_set_epi32(nonce[2] as i32, nonce[1] as i32, nonce[0] as i32, 0), self.v[3]);
        }
    }

    #[cfg(feature = "rand_core")]
    fn get_nonce(&self) -> [u32; 3] {
        let words: [u32; 4] = unsafe{ core::mem::transmute(self.v[3]) };
        [words[2], words[1], words[0]]
    }

    #[cfg(feature = "rand_core")]
    fn get_seed(&self) -> [u32; 8] {
        let seed1: [u32; 4] = unsafe { core::mem::transmute(self.v[1]) };
        let seed2: [u32; 4] = unsafe { core::mem::transmute(self.v[2]) };
        [seed1[3], seed1[2], seed1[1], seed1[0], seed2[3], seed2[2], seed2[1], seed2[0]]
    }

    fn increment_counter(&mut self, amount: i32) {
        unsafe {
            self.v[3] = _mm_add_epi32(self.v[3], _mm_set_epi32(0, 0, 0, amount))
        }
    }

    #[inline(always)]
    /// Generates a single block and blindly writes it to `dest_ptr`
    /// 
    /// # Safety
    /// `dest_ptr` must have at least 64 bytes available to be overwritten, or else it 
    /// could produce undefined behavior
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        let mut block_ptr = dest_ptr as *mut __m128i;
        for _i in 0..num_blocks {
            let res = rounds::<R>(&self.v);
            self.increment_counter(1);

            for i in 0..4 {
                _mm_storeu_si128(block_ptr.add(i), res[i]);
            }
            block_ptr = block_ptr.add(4);
        }
    }
}
// #[cfg(feature = "cipher")]
// impl<R: Rounds, V: Variant> Backend<R, V> {
//     #[inline]
//     #[target_feature(enable = "sse2")]
//     pub(crate) unsafe fn inner<F>(&mut self, f: F)
//     where
//         F: StreamClosure<BlockSize = U64>,
//     {
//         f.call(self)
//     }
// }

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for Backend<R, V> {
    type BlockSize = U64;
}
#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> ParBlocksSizeUser for Backend<R, V> {
    type ParBlocksSize = U1;
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamBackend for Backend<R, V> {
    #[inline(always)]
    fn gen_ks_block(&mut self, block: &mut Block) {
        // SAFETY: `Block` is a 64-byte array
        unsafe {
            self.write_ks_blocks(block.as_mut_ptr(), 1);
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn rounds<R: Rounds>(v: &[__m128i; 4]) -> [__m128i; 4] {
    let mut res = *v;
    for _ in 0..R::COUNT {
        double_quarter_round(&mut res);
    }

    for i in 0..4 {
        res[i] = _mm_add_epi32(res[i], v[i]);
    }

    res
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn double_quarter_round(v: &mut [__m128i; 4]) {
    add_xor_rot(v);
    rows_to_cols(v);
    add_xor_rot(v);
    cols_to_rows(v);
}

/// The goal of this function is to transform the state words from:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c0, c1, c2, c3]    [ 8,  9, 10, 11]
/// [d0, d1, d2, d3]    [12, 13, 14, 15]
/// ```
///
/// to:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b1, b2, b3, b0] == [ 5,  6,  7,  4]
/// [c2, c3, c0, c1]    [10, 11,  8,  9]
/// [d3, d0, d1, d2]    [15, 12, 13, 14]
/// ```
///
/// so that we can apply [`add_xor_rot`] to the resulting columns, and have it compute the
/// "diagonal rounds" (as defined in RFC 7539) in parallel. In practice, this shuffle is
/// non-optimal: the last state word to be altered in `add_xor_rot` is `b`, so the shuffle
/// blocks on the result of `b` being calculated.
///
/// We can optimize this by observing that the four quarter rounds in `add_xor_rot` are
/// data-independent: they only access a single column of the state, and thus the order of
/// the columns does not matter. We therefore instead shuffle the other three state words,
/// to obtain the following equivalent layout:
/// ```text
/// [a3, a0, a1, a2]    [ 3,  0,  1,  2]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c1, c2, c3, c0]    [ 9, 10, 11,  8]
/// [d2, d3, d0, d1]    [14, 15, 12, 13]
/// ```
///
/// See https://github.com/sneves/blake2-avx2/pull/4 for additional details. The earliest
/// known occurrence of this optimization is in floodyberry's SSE4 ChaCha code from 2014:
/// - https://github.com/floodyberry/chacha-opt/blob/0ab65cb99f5016633b652edebaf3691ceb4ff753/chacha_blocks_ssse3-64.S#L639-L643
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn rows_to_cols([a, _, c, d]: &mut [__m128i; 4]) {
    // c >>>= 32; d >>>= 64; a >>>= 96;
    *c = _mm_shuffle_epi32::<0b_00_11_10_01>(*c); // _MM_SHUFFLE(0, 3, 2, 1)
    *d = _mm_shuffle_epi32::<0b_01_00_11_10>(*d); // _MM_SHUFFLE(1, 0, 3, 2)
    *a = _mm_shuffle_epi32::<0b_10_01_00_11>(*a); // _MM_SHUFFLE(2, 1, 0, 3)
}

/// The goal of this function is to transform the state words from:
/// ```text
/// [a3, a0, a1, a2]    [ 3,  0,  1,  2]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c1, c2, c3, c0]    [ 9, 10, 11,  8]
/// [d2, d3, d0, d1]    [14, 15, 12, 13]
/// ```
///
/// to:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c0, c1, c2, c3]    [ 8,  9, 10, 11]
/// [d0, d1, d2, d3]    [12, 13, 14, 15]
/// ```
///
/// reversing the transformation of [`rows_to_cols`].
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn cols_to_rows([a, _, c, d]: &mut [__m128i; 4]) {
    // c <<<= 32; d <<<= 64; a <<<= 96;
    *c = _mm_shuffle_epi32::<0b_10_01_00_11>(*c); // _MM_SHUFFLE(2, 1, 0, 3)
    *d = _mm_shuffle_epi32::<0b_01_00_11_10>(*d); // _MM_SHUFFLE(1, 0, 3, 2)
    *a = _mm_shuffle_epi32::<0b_00_11_10_01>(*a); // _MM_SHUFFLE(0, 3, 2, 1)
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn add_xor_rot([a, b, c, d]: &mut [__m128i; 4]) {
    // a += b; d ^= a; d <<<= (16, 16, 16, 16);
    *a = _mm_add_epi32(*a, *b);
    *d = _mm_xor_si128(*d, *a);
    *d = _mm_xor_si128(_mm_slli_epi32::<16>(*d), _mm_srli_epi32::<16>(*d));

    // c += d; b ^= c; b <<<= (12, 12, 12, 12);
    *c = _mm_add_epi32(*c, *d);
    *b = _mm_xor_si128(*b, *c);
    *b = _mm_xor_si128(_mm_slli_epi32::<12>(*b), _mm_srli_epi32::<20>(*b));

    // a += b; d ^= a; d <<<= (8, 8, 8, 8);
    *a = _mm_add_epi32(*a, *b);
    *d = _mm_xor_si128(*d, *a);
    *d = _mm_xor_si128(_mm_slli_epi32::<8>(*d), _mm_srli_epi32::<24>(*d));

    // c += d; b ^= c; b <<<= (7, 7, 7, 7);
    *c = _mm_add_epi32(*c, *d);
    *b = _mm_xor_si128(*b, *c);
    *b = _mm_xor_si128(_mm_slli_epi32::<7>(*b), _mm_srli_epi32::<25>(*b));
}
