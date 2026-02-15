//! SSE2 backend.

#![allow(unsafe_op_in_unsafe_fn, reason = "needs triage")]
#![allow(clippy::cast_possible_truncation, reason = "needs triage")]
#![allow(clippy::cast_possible_wrap, reason = "needs triage")]
#![allow(clippy::cast_sign_loss, reason = "needs triage")]
#![allow(clippy::undocumented_unsafe_blocks, reason = "TODO")]

use crate::{Rounds, Variant};

#[cfg(feature = "rng")]
use crate::ChaChaCore;

#[cfg(feature = "cipher")]
use crate::{STATE_WORDS, chacha::Block};
#[cfg(feature = "cipher")]
use cipher::{
    BlockSizeUser, ParBlocksSizeUser, StreamCipherBackend, StreamCipherClosure,
    consts::{U4, U64},
};
use core::marker::PhantomData;

#[cfg(target_family = "wasm")]
use core::arch::wasm32::*;

const PAR_BLOCKS: usize = 4;

#[inline]
#[target_feature(enable = "simd128")]
#[cfg(feature = "cipher")]
pub(crate) unsafe fn inner<R, F, V>(state: &mut [u32; STATE_WORDS], f: F)
where
    R: Rounds,
    F: StreamCipherClosure<BlockSize = U64>,
    V: Variant,
{
    let state_ptr = state.as_ptr().cast::<v128>();
    let mut backend = Backend::<R, V> {
        v: [
            v128_load(state_ptr),
            v128_load(state_ptr.add(1)),
            v128_load(state_ptr.add(2)),
            v128_load(state_ptr.add(3)),
        ],
        _pd: PhantomData,
    };

    f.call(&mut backend);

    state[12] = u32x4_extract_lane::<0>(backend.v[3]);
    if size_of::<V::Counter>() == 8 {
        state[13] = u32x4_extract_lane::<1>(backend.v[3]);
    }
}

struct Backend<R: Rounds, V: Variant> {
    v: [v128; 4],
    _pd: PhantomData<(R, V)>,
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for Backend<R, V> {
    type BlockSize = U64;
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> ParBlocksSizeUser for Backend<R, V> {
    type ParBlocksSize = U4;
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> StreamCipherBackend for Backend<R, V> {
    #[inline(always)]
    fn gen_ks_block(&mut self, block: &mut Block) {
        unsafe {
            let res = rounds::<R, V>(&self.v);
            self.v[3] = match size_of::<V::Counter>() {
                4 => u32x4_add(self.v[3], u32x4(0, 0, 0, 1)),
                8 => u64x2_add(self.v[3], u64x2(0, 1)),
                _ => unreachable!(),
            };

            let block_ptr = block.as_mut_ptr().cast::<v128>();
            for i in 0..4 {
                v128_store(block_ptr.add(i), res[0][i]);
            }
        }
    }
    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut cipher::ParBlocks<Self>) {
        unsafe {
            let res = rounds::<R, V>(&self.v);
            self.v[3] = match size_of::<V::Counter>() {
                4 => u32x4_add(self.v[3], u32x4(0, 0, 0, PAR_BLOCKS as u32)),
                8 => u64x2_add(self.v[3], u64x2(0, PAR_BLOCKS as u64)),
                _ => unreachable!(),
            };

            let blocks_ptr = blocks.as_mut_ptr().cast::<v128>();
            for block in 0..PAR_BLOCKS {
                for i in 0..4 {
                    v128_store(blocks_ptr.add(i + block * PAR_BLOCKS), res[block][i]);
                }
            }
        }
    }
}

#[inline]
#[target_feature(enable = "simd128")]
#[cfg(feature = "rng")]
pub(crate) unsafe fn rng_inner<R, V>(core: &mut ChaChaCore<R, V>, dest_ptr: *mut u8, num_bytes: usize, fill_buffer: bool, buffer: &mut [u32; 64])
where
    R: Rounds,
    V: Variant,
{
    let state_ptr = core.state.as_ptr().cast::<__m128i>();
    let mut backend = Backend::<R, V> {
        v: [
            _mm_loadu_si128(state_ptr.add(0)),
            _mm_loadu_si128(state_ptr.add(1)),
            _mm_loadu_si128(state_ptr.add(2)),
            _mm_loadu_si128(state_ptr.add(3)),
        ],
        _pd: PhantomData,
    };

    for parblocks in 0..(num_bytes / 256) {
        backend.gen_ks_blocks(dest_ptr.add(256 * parblocks));
    }
    if fill_buffer {
        backend.gen_ks_blocks(buffer.as_mut_ptr() as *mut u8);
    }

    core.state[12] = _mm_cvtsi128_si32(backend.v[3]) as u32;
    core.state[13] = _mm_extract_epi32(backend.v[3], 1) as u32;
}

#[cfg(feature = "rng")]
impl<R: Rounds, V: Variant> Backend<R, V> {
    #[inline(always)]
    fn gen_ks_blocks(&mut self, dest_ptr: *mut u8) {
        const _: () = assert!(4 * PAR_BLOCKS * size_of::<__m128i>() == size_of::<[u32; 64]>());
        unsafe {
            let res = rounds::<R, V>(&self.v);
            self.v[3] = _mm_add_epi64(self.v[3], _mm_set_epi64x(0, PAR_BLOCKS as i64));

            let blocks_ptr = dest_ptr.cast::<__m128i>();
            for block in 0..PAR_BLOCKS {
                for i in 0..4 {
                    _mm_storeu_si128(blocks_ptr.add(i + block * PAR_BLOCKS), res[block][i]);
                }
            }
        }
    }
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn rounds<R: Rounds, V: Variant>(v: &[v128; 4]) -> [[v128; 4]; PAR_BLOCKS] {
    let mut res = [*v; 4];
    for block in 1..PAR_BLOCKS {
        res[block][3] = match size_of::<V::Counter>() {
            4 => u32x4_add(res[block][3], u32x4(0, 0, 0, block as u32)),
            8 => u64x2_add(res[block][3], u64x2(0, block as u64)),
            _ => unreachable!(),
        }
    }

    for _ in 0..R::COUNT {
        double_quarter_round(&mut res);
    }

    for block in 0..PAR_BLOCKS {
        for i in 0..3 {
            res[block][i] = u32x4_add(res[block][i], v[i]);
        }
        let ctr = match size_of::<V::Counter>() {
            4 => u32x4_add(v[3], u32x4(0, 0, 0, block as u32)),
            8 => u64x2_add(v[3], u64x2(0, block as u64)),
            _ => unreachable!(),
        };
        res[block][3] = u32x4_add(res[block][3], ctr);
    }

    res
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn double_quarter_round(v: &mut [[v128; 4]; PAR_BLOCKS]) {
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
#[target_feature(enable = "simd128")]
unsafe fn rows_to_cols(blocks: &mut [[v128; 4]; PAR_BLOCKS]) {
    for [a, _, c, d] in blocks.iter_mut() {
        // c >>>= 32; d >>>= 64; a >>>= 96;
        *c = u32x4_shuffle::<1, 2, 3, 0>(*c, *c);
        *d = u32x4_shuffle::<2, 3, 0, 1>(*d, *d);
        *a = u32x4_shuffle::<3, 0, 1, 2>(*a, *a);
        //*c = _mm_shuffle_epi32(*c, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
        //*d = _mm_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
        //*a = _mm_shuffle_epi32(*a, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
    }
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
#[target_feature(enable = "simd128")]
unsafe fn cols_to_rows(blocks: &mut [[v128; 4]; PAR_BLOCKS]) {
    for [a, _, c, d] in blocks.iter_mut() {
        // c <<<= 32; d <<<= 64; a <<<= 96;
        *c = u32x4_shuffle::<3, 0, 1, 2>(*c, *c);
        *d = u32x4_shuffle::<2, 3, 0, 1>(*d, *d);
        *a = u32x4_shuffle::<1, 2, 3, 0>(*a, *a);
        //*c = _mm_shuffle_epi32(*c, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
        //*d = _mm_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
        //*a = _mm_shuffle_epi32(*a, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
    }
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn add_xor_rot(blocks: &mut [[v128; 4]; PAR_BLOCKS]) {
    for [a, b, c, d] in blocks.iter_mut() {
        // a += b; d ^= a; d <<<= (16, 16, 16, 16);
        *a = u32x4_add(*a, *b);
        *d = v128_xor(*d, *a);
        *d = v128_xor(u32x4_shl(*d, 16), u32x4_shr(*d, 16));

        // c += d; b ^= c; b <<<= (12, 12, 12, 12);
        *c = u32x4_add(*c, *d);
        *b = v128_xor(*b, *c);
        *b = v128_xor(u32x4_shl(*b, 12), u32x4_shr(*b, 20));

        // a += b; d ^= a; d <<<= (8, 8, 8, 8);
        *a = u32x4_add(*a, *b);
        *d = v128_xor(*d, *a);
        *d = v128_xor(u32x4_shl(*d, 8), u32x4_shr(*d, 24));

        // c += d; b ^= c; b <<<= (7, 7, 7, 7);
        *c = u32x4_add(*c, *d);
        *b = v128_xor(*b, *c);
        *b = v128_xor(u32x4_shl(*b, 7), u32x4_shr(*b, 25));
    }
}
