use crate::{Rounds, STATE_WORDS, Variant};

#[cfg(feature = "cipher")]
use crate::chacha::Block;
use core::marker::PhantomData;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(feature = "cipher")]
use cipher::{StreamBackend, 
    consts::{U4, U64}, 
    ParBlocks, ParBlocksSizeUser, BlockSizeUser, StreamClosure
};

use super::BackendType;

/// Number of blocks processed in parallel.
const PAR_BLOCKS: usize = 4;
/// Number of `__m256i` to store parallel blocks.
const N: usize = PAR_BLOCKS / 2;

/// A constant for applying masks to SIMD registers
const I32_ONES: i32 = 0xFFFF_FFFFu32 as i32;

/// Extracts a single block of output from a [__m256i; 4].
/// ### Parameters
/// - `$block_ptr`: a `*mut __m128i` to write a block to
/// - `$block_pair`: a `[__m256i; 4]` that contains 2 blocks of output
/// - `$block_index`: a `0` or `1` to determine which block you want to extract
/// ### Safety
/// - Ensure that `$block_ptr` is suitable for receiving 64 bytes of data
macro_rules! extract_1_block {
    ($block_ptr:expr, $block_pair:expr, $block_index:literal) => {
        let t: [__m128i; 8] = core::mem::transmute($block_pair);
        for i in 0..4 {
            _mm_storeu_si128($block_ptr.add(i), t[(i << 1) + $block_index]);
        }
        $block_ptr = $block_ptr.add(4);
    };
}

/// Extracts 2 blocks of output from a [__m256i; 4].
/// ### Parameters
/// - `$block_ptr`: a `*mut __m128i` to write a block to
/// - `$block_pair`: a `[__m256i; 4]` that contains 2 blocks of output
/// ### Safety
/// - Ensure that `$block_ptr` is suitable for receiving 128 bytes of data
macro_rules! extract_2_blocks {
    ($block_ptr:expr, $block_pair:expr) => {
        let t: [__m128i; 8] = core::mem::transmute($block_pair);
        for i in 0..4 {
            _mm_storeu_si128($block_ptr.add(i), t[i<<1]);
            _mm_storeu_si128($block_ptr.add(4+i), t[(i<<1) + 1]);
        }
        $block_ptr = $block_ptr.add(8);
    };
}

pub(crate) struct Backend<R: Rounds, V: Variant> {
    v: [__m256i; 3],
    ctr: [__m256i; N],
    results: [[__m256i; 4]; N],
    block: usize,
    _pd: PhantomData<R>,
    _variant: PhantomData<V>
}

impl<R: Rounds, V: Variant> Clone for Backend<R, V> {
    fn clone(&self) -> Self {
        Self {
            v: self.v,
            ctr: self.ctr,
            results: self.results,
            block: self.block,
            _pd: self._pd,
            _variant: self._variant
        }
    }
}

impl<R: Rounds, V: Variant> BackendType for Backend<R, V> {
    const PAR_BLOCKS: usize = 4;

    #[inline]
    fn new(state: &[u32; STATE_WORDS]) -> Self {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            let v = [
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(0))),
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(1))),
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(2))),
            ];
            let mut c = _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(3)));
            c = _mm256_add_epi32(c, _mm256_setr_epi32(0, 0, 0, 0, 1, 0, 0, 0));
            let mut ctr = [c; N];
            for i in 0..N {
                ctr[i] = c;
                c = _mm256_add_epi32(c, _mm256_setr_epi32(2, 0, 0, 0, 2, 0, 0, 0));
            }
            Backend::<R, V> {
                v,
                ctr,
                results: [[_mm256_setzero_si256(); 4]; N],
                block: 4,
                _pd: PhantomData,
                _variant: PhantomData
            }
        }
    }

    /// Updates the 4th row of the ChaCha State
    fn update_state(&mut self, state: &[u32]) {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            let mut c = _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(3)));
            c = _mm256_add_epi32(c, _mm256_setr_epi32(0, 0, 0, 0, 1, 0, 0, 0));
            for i in 0..N {
                self.ctr[i] = c;
                c = _mm256_add_epi32(c, _mm256_setr_epi32(2, 0, 0, 0, 2, 0, 0, 0));
            }
            self.block = 4;
        }
    }

    /// Increments the counter by `amount`
    #[inline]
    fn increment_counter(&mut self, amount: i32) {
        unsafe {
            for c in self.ctr.iter_mut() {
                *c = _mm256_add_epi32(*c, _mm256_setr_epi32(amount, 0, 0, 0, amount, 0, 0, 0));
            }
        }
    }
    
    //#[inline]
    /// Generates `num_blocks` blocks and blindly writes them to `dest_ptr` recursively
    /// 
    /// # Safety
    /// `dest_ptr` must have at least `num_blocks * 64` bytes available to be overwritten, or else it 
    /// could result in a segmentation fault or undesired behavior
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, mut num_blocks: usize) {
        let mut _block_ptr = dest_ptr as *mut __m128i;
        //while num_blocks > 0 {
        if self.block == Self::PAR_BLOCKS {
            self.rounds();
            self.block = 0;
        }

        // if (self.block % 2 == 0) to check if either `extract_2_blocks` or `extract_first_block` can be used
        if self.block & 0b01 == 0 {
            if num_blocks >= 2 {
                extract_2_blocks!(_block_ptr, self.results[self.block >> 1]);
                self.block += 2;
                //num_blocks -= 2;
                self.write_ks_blocks(_block_ptr as *mut u8, num_blocks - 2);
            } else if num_blocks == 1 {
                extract_1_block!(_block_ptr, self.results[self.block >> 1], 0);
                self.block += 1;
                //num_blocks -= 1;
            } // else: num_blocks == 0, recursion ends
        } else if num_blocks > 0 {
            // self.block is either 1 or 3
            extract_1_block!(_block_ptr, self.results[self.block >> 1], 1);
            self.block += 1;
            //num_blocks -= 1;
            self.write_ks_blocks(_block_ptr as *mut u8, num_blocks - 1);
        } // else: num_blocks == 0, recursion ends
        //}
    }

    #[cfg(feature = "rng")]
    #[inline]
    fn rng_inner(&mut self, mut dest_ptr: *mut u8, mut num_blocks: usize) {
        // limiting recursion depth to a maximum of 3 recursive calls to try 
        // to reduce memory usage
        unsafe {
            while num_blocks > 4 {
                self.write_ks_blocks(dest_ptr, 4);
                dest_ptr = dest_ptr.add(256);
                num_blocks -= 4;
            }
            if num_blocks > 0 {
                self.write_ks_blocks(dest_ptr, num_blocks)
            }
        }
    }
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
impl<R: Rounds, V: Variant> StreamBackend for Backend<R, V> {
    #[inline(always)]
    fn gen_ks_block(&mut self, dest_ptr: &mut Block) {
        unsafe {
            self.write_ks_blocks(dest_ptr.as_mut_ptr(), 1);
        }
    }

    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut ParBlocks<Self>) {
        // SAFETY: `ParBlocks` is a 256-byte 2D array.
        unsafe {
            self.write_ks_blocks(blocks.as_mut_ptr() as *mut u8, 4);
        }
    }
}

impl<R: Rounds, V: Variant> Backend<R, V> {
    #[inline]
    #[cfg(feature = "cipher")]
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn inner<F>(&mut self, state_counter: &mut u32, f: F) 
    where
        R: Rounds,
        F: StreamClosure<BlockSize = U64>,
        V: Variant
    {
        f.call(self);
        *state_counter = _mm256_extract_epi32::<0>(self.ctr[0]) as u32;
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    /// Overwrites the `self.results` buffer with PAR_BLOCKS results
    unsafe fn rounds(&mut self) {
        for i in 0..N {
            self.results[i] = [self.v[0], self.v[1], self.v[2], self.ctr[i]];
        }
        for _ in 0..R::COUNT {
            double_quarter_round(&mut self.results);
        }

        for i in 0..N {
            for j in 0..3 {
                self.results[i][j] = _mm256_add_epi32(self.results[i][j], self.v[j]);
            }
            self.results[i][3] = _mm256_add_epi32(self.results[i][3], self.ctr[i]);
        }

        self.increment_counter(Self::PAR_BLOCKS as i32);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn double_quarter_round(v: &mut [[__m256i; 4]; N]) {
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
#[target_feature(enable = "avx2")]
unsafe fn rows_to_cols(vs: &mut [[__m256i; 4]; N]) {
    // c >>>= 32; d >>>= 64; a >>>= 96;
    for [a, _, c, d] in vs {
        *c = _mm256_shuffle_epi32::<0b_00_11_10_01>(*c); // _MM_SHUFFLE(0, 3, 2, 1)
        *d = _mm256_shuffle_epi32::<0b_01_00_11_10>(*d); // _MM_SHUFFLE(1, 0, 3, 2)
        *a = _mm256_shuffle_epi32::<0b_10_01_00_11>(*a); // _MM_SHUFFLE(2, 1, 0, 3)
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
#[target_feature(enable = "avx2")]
unsafe fn cols_to_rows(vs: &mut [[__m256i; 4]; N]) {
    // c <<<= 32; d <<<= 64; a <<<= 96;
    for [a, _, c, d] in vs {
        *c = _mm256_shuffle_epi32::<0b_10_01_00_11>(*c); // _MM_SHUFFLE(2, 1, 0, 3)
        *d = _mm256_shuffle_epi32::<0b_01_00_11_10>(*d); // _MM_SHUFFLE(1, 0, 3, 2)
        *a = _mm256_shuffle_epi32::<0b_00_11_10_01>(*a); // _MM_SHUFFLE(0, 3, 2, 1)
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn add_xor_rot(vs: &mut [[__m256i; 4]; N]) {
    let rol16_mask = _mm256_set_epi64x(
        0x0d0c_0f0e_0908_0b0a,
        0x0504_0706_0100_0302,
        0x0d0c_0f0e_0908_0b0a,
        0x0504_0706_0100_0302,
    );
    let rol8_mask = _mm256_set_epi64x(
        0x0e0d_0c0f_0a09_080b,
        0x0605_0407_0201_0003,
        0x0e0d_0c0f_0a09_080b,
        0x0605_0407_0201_0003,
    );

    // a += b; d ^= a; d <<<= (16, 16, 16, 16);
    for [a, b, _, d] in vs.iter_mut() {
        *a = _mm256_add_epi32(*a, *b);
        *d = _mm256_xor_si256(*d, *a);
        *d = _mm256_shuffle_epi8(*d, rol16_mask);
    }

    // c += d; b ^= c; b <<<= (12, 12, 12, 12);
    for [_, b, c, d] in vs.iter_mut() {
        *c = _mm256_add_epi32(*c, *d);
        *b = _mm256_xor_si256(*b, *c);
        *b = _mm256_xor_si256(_mm256_slli_epi32::<12>(*b), _mm256_srli_epi32::<20>(*b));
    }

    // a += b; d ^= a; d <<<= (8, 8, 8, 8);
    for [a, b, _, d] in vs.iter_mut() {
        *a = _mm256_add_epi32(*a, *b);
        *d = _mm256_xor_si256(*d, *a);
        *d = _mm256_shuffle_epi8(*d, rol8_mask);
    }

    // c += d; b ^= c; b <<<= (7, 7, 7, 7);
    for [_, b, c, d] in vs.iter_mut() {
        *c = _mm256_add_epi32(*c, *d);
        *b = _mm256_xor_si256(*b, *c);
        *b = _mm256_xor_si256(_mm256_slli_epi32::<7>(*b), _mm256_srli_epi32::<25>(*b));
    }
}

/*
#[cfg(test)]
mod test {
    use crate::{R20, variants::Ietf};

    use super::*;

    #[test]
    fn test_set_block_pos() {
        let mut test_state: [u32; 16] = 
        [
                0x0504_0706, 0x0605_0407, 0x0d0c_0f0e, 0x0e0d_0c0f,
                384823354u32, 12424524u32, 3138953u32, 8338348u32,
                1234u32, 93849u32, 1398348u32, 11111u32,
                0u32, 0u32, 0u32, 0u32
        ];
        let mut backend = Backend::<R20, Ietf>::new(&mut test_state);

        let block_pos = 15;
        backend.set_block_pos(block_pos);

        let ctr0 = backend.ctr[0];
        let ctr1 = backend.ctr[1];
        unsafe {
            let ctr00 = _mm256_extract_epi32::<0>(ctr0) as u32;
            let ctr01 = _mm256_extract_epi32::<4>(ctr0) as u32;
            let ctr02 = _mm256_extract_epi32::<0>(ctr1) as u32;
            let ctr03 = _mm256_extract_epi32::<4>(ctr1) as u32;

            assert_eq!(
                [
                    ctr00, 
                    ctr01, 
                    ctr02, 
                    ctr03
                    ], 
                [
                    block_pos, 
                    block_pos + 1, 
                    block_pos + 2, 
                    block_pos + 3
                    ]
            );
        }

        assert_eq!(backend.get_block_pos(), 15);

        let block_pos: u32 = (2 << 31) + 3432;
        backend.set_block_pos(block_pos);

        let ctr0 = backend.ctr[0];
        let ctr1 = backend.ctr[1];

        unsafe {
            let ctr00 = _mm256_extract_epi32::<0>(ctr0) as u32;
            let ctr01 = _mm256_extract_epi32::<4>(ctr0) as u32;
            let ctr02 = _mm256_extract_epi32::<0>(ctr1) as u32;
            let ctr03 = _mm256_extract_epi32::<4>(ctr1) as u32;

            assert_eq!(
                [
                    ctr00, 
                    ctr01, 
                    ctr02, 
                    ctr03
                    ], 
                [
                    block_pos, 
                    block_pos + 1, 
                    block_pos + 2, 
                    block_pos + 3
                    ]
            );
        }
        assert_eq!(backend.get_block_pos(), block_pos);
    }

    #[test]
    /// this test is to figure out how the state is represented in an __m128i
    fn debugging_simd() {
        let mut state: [u32; 16] = 
        [
            1111, 2222, 3333, 4444,
            5555, 6666, 7777, 8888,
            9999, 1010, 1111, 1212,
            1313, 1414, 1515, 1616
        ];
        let mut backend = Backend::<R20, Ietf>::new(&mut state);
        unsafe {
            let ctr_row = backend.ctr[0];
            // confirm that this is where the 32-bit counter is
            let ctr = _mm256_extract_epi32::<0>(ctr_row);
            assert_eq!(ctr, 1313);
            let ctr_2 = _mm256_extract_epi32::<4>(ctr_row);
            assert_eq!(ctr_2, 1314);

            // confirm that this sets the counters to 0, and preserves the nonce
            let mask = _mm256_set_epi32(0, 0, 0, I32_ONES, 0, 0, 0, I32_ONES);
            for i in 0..N {
                backend.ctr[i] = _mm256_andnot_si256(mask, backend.ctr[i]);
            }

            for i in 0..N {
                let ctr_row = backend.ctr[i];
                let ctr = _mm256_extract_epi32::<0>(ctr_row);
                let nonce_1 = _mm256_extract_epi32::<1>(ctr_row);
                let nonce_2 = _mm256_extract_epi32::<2>(ctr_row);
                let nonce_3 = _mm256_extract_epi32::<3>(ctr_row);
                let ctr_2 = _mm256_extract_epi32::<4>(ctr_row);
                let nonce_4 = _mm256_extract_epi32::<5>(ctr_row);
                let nonce_5 = _mm256_extract_epi32::<6>(ctr_row);
                let nonce_6 = _mm256_extract_epi32::<7>(ctr_row);
                assert_eq!(ctr, 0);
                assert_eq!(nonce_1, 1414);
                assert_eq!(nonce_2, 1515);
                assert_eq!(nonce_3, 1616);
                assert_eq!(ctr_2, 0);
                assert_eq!(nonce_4, nonce_1);
                assert_eq!(nonce_5, nonce_2);
                assert_eq!(nonce_6, nonce_3);
            }

            // add a counter
            for i in 0..N {
                backend.ctr[i] = _mm256_andnot_si256(mask, backend.ctr[i]);
                backend.ctr[i] = _mm256_add_epi32(
                    backend.ctr[i],
                    _mm256_setr_epi32(2565 + 2*i as i32, 0, 0, 0, 2565+ 2*i as i32 + 1, 0, 0, 0)
                );
            }

            for i in 0..N {
                let ctr_row = backend.ctr[i];
                let ctr = _mm256_extract_epi32::<0>(ctr_row);
                let nonce_1 = _mm256_extract_epi32::<1>(ctr_row);
                let nonce_2 = _mm256_extract_epi32::<2>(ctr_row);
                let nonce_3 = _mm256_extract_epi32::<3>(ctr_row);
                let ctr_2 = _mm256_extract_epi32::<4>(ctr_row);
                let nonce_4 = _mm256_extract_epi32::<5>(ctr_row);
                let nonce_5 = _mm256_extract_epi32::<6>(ctr_row);
                let nonce_6 = _mm256_extract_epi32::<7>(ctr_row);
                assert_eq!(ctr, 2565 + i as i32 * 2);
                assert_eq!(nonce_1, 1414);
                assert_eq!(nonce_2, 1515);
                assert_eq!(nonce_3, 1616);
                assert_eq!(ctr_2, 2565 + i as i32 * 2 + 1);
                assert_eq!(nonce_4, nonce_1);
                assert_eq!(nonce_5, nonce_2);
                assert_eq!(nonce_6, nonce_3);
            }

            // confirm that we can do the same to the nonce
            backend = Backend::new(&mut state);
            for i in 0..N {
                backend.ctr[i] = _mm256_and_si256(backend.ctr[i], mask);
            }

            for i in 0..N {
                let ctr_row = backend.ctr[i];
                let ctr = _mm256_extract_epi32::<0>(ctr_row);
                let nonce_1 = _mm256_extract_epi32::<1>(ctr_row);
                let nonce_2 = _mm256_extract_epi32::<2>(ctr_row);
                let nonce_3 = _mm256_extract_epi32::<3>(ctr_row);
                let ctr_2 = _mm256_extract_epi32::<4>(ctr_row);
                let nonce_4 = _mm256_extract_epi32::<5>(ctr_row);
                let nonce_5 = _mm256_extract_epi32::<6>(ctr_row);
                let nonce_6 = _mm256_extract_epi32::<7>(ctr_row);
                assert_eq!(ctr, 1313 + i as i32 * 2);
                assert_eq!(nonce_1, 0);
                assert_eq!(nonce_2, 0);
                assert_eq!(nonce_3, 0);
                assert_eq!(ctr_2, 1313 + i as i32 * 2 + 1);
                assert_eq!(nonce_4, nonce_1);
                assert_eq!(nonce_5, nonce_2);
                assert_eq!(nonce_6, nonce_3);
            }

            // add a nonce
            for i in 0..N {
                backend.ctr[i] = _mm256_and_si256(backend.ctr[i], mask);
                backend.ctr[i] = _mm256_add_epi32(
                    backend.ctr[i], 
                    _mm256_setr_epi32(0, 1414, 1515, 1616, 0, 1414, 1515, 1616)
                );
            }

            for i in 0..N {
                let ctr_row = backend.ctr[i];
                let ctr = _mm256_extract_epi32::<0>(ctr_row);
                let nonce_1 = _mm256_extract_epi32::<1>(ctr_row);
                let nonce_2 = _mm256_extract_epi32::<2>(ctr_row);
                let nonce_3 = _mm256_extract_epi32::<3>(ctr_row);
                let ctr_2 = _mm256_extract_epi32::<4>(ctr_row);
                let nonce_4 = _mm256_extract_epi32::<5>(ctr_row);
                let nonce_5 = _mm256_extract_epi32::<6>(ctr_row);
                let nonce_6 = _mm256_extract_epi32::<7>(ctr_row);
                assert_eq!(ctr, 1313 + i as i32 * 2);
                assert_eq!(nonce_1, 1414);
                assert_eq!(nonce_2, 1515);
                assert_eq!(nonce_3, 1616);
                assert_eq!(ctr_2, 1313 + i as i32 * 2 + 1);
                assert_eq!(nonce_4, nonce_1);
                assert_eq!(nonce_5, nonce_2);
                assert_eq!(nonce_6, nonce_3);
            }
        }
    }
}
*/