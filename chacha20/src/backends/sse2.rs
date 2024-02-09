use crate::{Rounds, Variant, STATE_WORDS};
use core::marker::PhantomData;

#[cfg(feature = "cipher")]
use crate::chacha::Block;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(feature = "cipher")]
use cipher::{
    consts::{U1, U64},
    BlockSizeUser, ParBlocksSizeUser, StreamBackend, StreamClosure,
};

#[derive(Clone)]
pub(super) struct Backend<R: Rounds, V: Variant> {
    v: [__m128i; 4],
    res: [__m128i; 4],
    _pd: PhantomData<R>,
    _variant: PhantomData<V>,
}

impl<R: Rounds, V: Variant> Backend<R, V> {
    #[inline(always)]
    pub(super) fn new(state: &[u32; STATE_WORDS]) -> Self {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            Self {
                v: [
                    _mm_loadu_si128(state_ptr.add(0)),
                    _mm_loadu_si128(state_ptr.add(1)),
                    _mm_loadu_si128(state_ptr.add(2)),
                    _mm_loadu_si128(state_ptr.add(3)),
                ],
                res: [
                    _mm_setzero_si128(),
                    _mm_setzero_si128(),
                    _mm_setzero_si128(),
                    _mm_setzero_si128(),
                ],
                _pd: PhantomData,
                _variant: PhantomData,
            }
        }
    }

    #[inline(always)]
    pub(super) fn update_state(&mut self, state: &[u32]) {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            self.v[3] = _mm_loadu_si128(state_ptr.add(3))
        }
    }

    #[inline(always)]
    fn increment_counter(&mut self, amount: i32) {
        unsafe { self.v[3] = _mm_add_epi32(self.v[3], _mm_set_epi32(0, 0, 0, amount)) }
    }

    /// Generates 4 blocks of data and writes it to the buffer
    #[inline(always)]
    #[cfg(feature = "rng")]
    pub(super) fn generate(&mut self, buffer: &mut [u32; 64]) {
        let mut dest_ptr = buffer.as_mut_ptr() as *mut __m128i;
        for _i in 0..4 {
            unsafe {
                self.rounds();
                self.increment_counter(1);

                for i in 0..4 {
                    _mm_storeu_si128(dest_ptr.add(i), self.res[i]);
                }
                dest_ptr = dest_ptr.add(4);
            }
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn rounds(&mut self) {
        self.res = self.v;
        for _ in 0..R::COUNT {
            double_quarter_round(&mut self.res);
        }

        for i in 0..4 {
            self.res[i] = _mm_add_epi32(self.res[i], self.v[i]);
        }
    }

    #[inline]
    #[cfg(feature = "cipher")]
    #[target_feature(enable = "sse2")]
    pub(super) unsafe fn inner<F>(&mut self, state_counter: &mut u32, f: F)
    where
        R: Rounds,
        F: StreamClosure<BlockSize = U64>,
        V: Variant,
    {
        f.call(self);
        *state_counter = _mm_extract_epi32::<0>(self.v[3]) as u32;
    }
}

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
        unsafe {
            let dest_ptr = block.as_mut_ptr() as *mut __m128i;

            self.rounds();
            self.increment_counter(1);

            for i in 0..4 {
                _mm_storeu_si128(dest_ptr.add(i), self.res[i]);
            }
        }
    }
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
