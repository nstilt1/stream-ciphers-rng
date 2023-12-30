use crate::{Rounds, STATE_WORDS, Variant};

#[cfg(feature = "cipher")]
use crate::chacha::Block;
use core::marker::PhantomData;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(feature = "cipher")]
use cipher::{StreamBackend, StreamClosure, 
    consts::{U4, U64}, 
    ParBlocks, ParBlocksSizeUser, BlockSizeUser
};

use super::BackendType;

/// Number of blocks processed in parallel.
const PAR_BLOCKS: usize = 4;
/// Number of `__m256i` to store parallel blocks.
const N: usize = PAR_BLOCKS / 2;

/// Extracts 1 block of output from a [__m256i; 4]
macro_rules! extract_1_block {
    ($block_ptr:expr, $block_pair:expr) => {
        let t: [__m128i; 8] = core::mem::transmute($block_pair);
        for i in 0..4 {
            _mm_storeu_si128($block_ptr.add(i), t[i << 1]);
        }
    };
}

/// Extracts 2 blocks of output from a [__m256i; 4]
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

#[derive(Clone)]
pub(crate) struct Backend<R: Rounds, V: Variant> {
    v: [__m256i; 3],
    ctr: [__m256i; N],
    _pd: PhantomData<R>,
    _variant: PhantomData<V>
}

impl<R: Rounds, V: Variant> BackendType for Backend<R, V> {
    const PAR_BLOCKS: usize = 4;

    #[inline]
    fn new(state: &mut [u32; STATE_WORDS]) -> Self {
        unsafe {
            let state_ptr = state.as_ptr() as *const __m128i;
            let v = [
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(0))),
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(1))),
                _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(2))),
            ];
            let mut c = _mm256_broadcastsi128_si256(_mm_loadu_si128(state_ptr.add(3)));
            c = _mm256_add_epi32(c, _mm256_set_epi32(0, 0, 0, 1, 0, 0, 0, 0));
            let mut ctr = [c; N];
            for i in 0..N {
                ctr[i] = c;
                c = _mm256_add_epi32(c, _mm256_set_epi32(0, 0, 0, 2, 0, 0, 0, 2));
            }
            Backend::<R, V> {
                v,
                ctr,
                _pd: PhantomData,
                _variant: PhantomData
            }
        }
    }

    /// Increments the counter by `amount`
    #[inline]
    fn increment_counter(&mut self, amount: i32) {
        unsafe {
            for c in self.ctr.iter_mut() {
                *c = _mm256_add_epi32(*c, _mm256_set_epi32(0, 0, 0, amount, 0, 0, 0, amount));
            }
        }
    }

    fn get_block_pos(&self) -> u32 {
        unsafe { 
            let mut register_data: [u32; 4] = core::mem::transmute(_mm256_extracti128_si256(self.ctr[N-1]));
            register_data[3]
        }
    }

    fn set_block_pos(&mut self, amount: u32) {
        let i32_max = 0xFFFF_FFFF;
        unsafe {
            // apply a mask to the counters to set them to 0
            let mut mask = _mm256_set_epi32(0, 0, 0, i32_max, 0, 0, 0, i32_max);
            
            for i in 0..N {
                self.ctr[i] = _mm256_andnot_si256(self.ctr[i], mask);
                self.ctr[i] = _mm256_add_epi32(self.ctr[i], _mm256_set_epi32(0, 0, 0, amount as i32, 0, 0, 0, amount as i32));
                
                // increasing the counter in separate operations to avoid an i32 overflow before the  
                // `amount + ix` takes place
                self.ctr[i] = _mm256_add_epi32(self.ctr[i], _mm256_set_epi32(0, 0, 0, i as i32 * 2 + 1, 0, 0, 0, i as i32 * 2));
            }
        }
    }

    #[inline(always)]
    /// Generates `num_blocks` blocks and blindly writes them to `dest_ptr`
    /// 
    /// # Safety
    /// `dest_ptr` must have at least `num_blocks * 64` bytes available to be overwritten, or else it 
    /// could produce undefined behavior
    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        assert!(num_blocks <= PAR_BLOCKS, "num_blocks in avx2::write_par_ks_blocks must be <= 4");

        let vs = rounds::<R>(&self.v, &self.ctr);
        
        self.increment_counter(num_blocks as i32);

        let mut _block_ptr = dest_ptr as *mut __m128i;

        // extract up to 4 blocks
        if num_blocks >= 2 {
            extract_2_blocks!(_block_ptr, vs[0]);
            if num_blocks == 4 {
                extract_2_blocks!(_block_ptr, vs[1]);
            }else if num_blocks == 3 {
                extract_1_block!(_block_ptr, vs[1]);
            }
        }else if num_blocks == 1 {
            extract_1_block!(_block_ptr, vs[0]);
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



#[inline]
#[target_feature(enable = "avx2")]
unsafe fn rounds<R: Rounds>(v: &[__m256i; 3], c: &[__m256i; N]) -> [[__m256i; 4]; N] {
    let mut vs: [[__m256i; 4]; N] = [[_mm256_setzero_si256(); 4]; N];
    for i in 0..N {
        vs[i] = [v[0], v[1], v[2], c[i]];
    }
    for _ in 0..R::COUNT {
        double_quarter_round(&mut vs);
    }

    for i in 0..N {
        for j in 0..3 {
            vs[i][j] = _mm256_add_epi32(vs[i][j], v[j]);
        }
        vs[i][3] = _mm256_add_epi32(vs[i][3], c[i]);
    }

    vs
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
        *c = _mm256_shuffle_epi32(*c, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
        *d = _mm256_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
        *a = _mm256_shuffle_epi32(*a, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
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
        *c = _mm256_shuffle_epi32(*c, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
        *d = _mm256_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
        *a = _mm256_shuffle_epi32(*a, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
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
        *b = _mm256_xor_si256(_mm256_slli_epi32(*b, 12), _mm256_srli_epi32(*b, 20));
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
        *b = _mm256_xor_si256(_mm256_slli_epi32(*b, 7), _mm256_srli_epi32(*b, 25));
    }
}
