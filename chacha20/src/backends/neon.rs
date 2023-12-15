//! NEON-optimized implementation for aarch64 CPUs.
//!
//! Adapted from the Crypto++ `chacha_simd` implementation by Jack Lloyd and
//! Jeffrey Walton (public domain).

use crate::Rounds;
use core::{arch::aarch64::*, marker::PhantomData};

#[cfg(feature = "rand_core")]
use crate::{variants::Variant, ChaChaCore};

#[cfg(feature = "cipher")]
use crate::{chacha::Block, STATE_WORDS};
#[cfg(feature = "cipher")]
use cipher::{
    consts::{U4, U64},
    BlockSizeUser, ParBlocks, ParBlocksSizeUser, StreamBackend, StreamClosure,
};

struct Backend<R: Rounds> {
    state: [uint32x4_t; 4],
    _pd: PhantomData<R>,
}
#[cfg(feature = "cipher")]
impl<R: Rounds> BlockSizeUser for Backend<R> {
    type BlockSize = U64;
}
#[cfg(feature = "cipher")]
impl<R: Rounds> ParBlocksSizeUser for Backend<R> {
    type ParBlocksSize = U4;
}

#[inline]
#[cfg(feature = "cipher")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn inner<R, F>(state: &mut [u32; STATE_WORDS], f: F)
where
    R: Rounds,
    F: StreamClosure<BlockSize = U64>,
{
    let mut backend = Backend::<R> {
        state: [
            vld1q_u32(state.as_ptr().offset(0)),
            vld1q_u32(state.as_ptr().offset(4)),
            vld1q_u32(state.as_ptr().offset(8)),
            vld1q_u32(state.as_ptr().offset(12)),
        ],
        _pd: PhantomData,
    };

    f.call(&mut backend);

    vst1q_u32(state.as_mut_ptr().offset(12), backend.state[3]);
}

macro_rules! add64 {
    ($a:expr, $b:expr) => {
        vreinterpretq_u32_u64(vaddq_u64(
            vreinterpretq_u64_u32($a),
            vreinterpretq_u64_u32($b),
        ))
    };
}

#[cfg(feature = "cipher")]
impl<R: Rounds> StreamBackend for Backend<R> {
    #[inline(always)]
    fn gen_ks_block(&mut self, block: &mut Block) {
        let state3 = self.state[3];
        let mut par = ParBlocks::<Self>::default();
        self.gen_par_ks_blocks(&mut par);
        *block = par[0];
        unsafe {
            self.state[3] = add64!(state3, vld1q_u32([1, 0, 0, 0].as_ptr()));
        }
    }

    #[inline(always)]
    fn gen_par_ks_blocks(&mut self, blocks: &mut ParBlocks<Self>) {
        self.write_par_ks_blocks(blocks.as_mut_ptr() as *mut u8);
    }
}

#[inline]
#[cfg(feature = "rand_core")]
#[target_feature(enable = "neon")]
/// This is essentially the same as the `inner` method except it takes a pointer
/// and it only calls the `gen_par_ks_blocks` method.
pub(crate) unsafe fn rng_inner<R, V>(core: &mut ChaChaCore<R, V>, dest: *mut u8)
where
    R: Rounds,
    V: Variant,
{
    assert!(!dest.is_null(), "Pointer must not be null");
    let mut backend = Backend::<R> {
        state: [
            vld1q_u32(core.state.as_ptr().offset(0)),
            vld1q_u32(core.state.as_ptr().offset(4)),
            vld1q_u32(core.state.as_ptr().offset(8)),
            vld1q_u32(core.state.as_ptr().offset(12)),
        ],
        _pd: PhantomData,
    };

    backend.write_par_ks_blocks(dest);

    vst1q_u32(core.state.as_mut_ptr().offset(12), backend.state[3]);
}

impl<R: Rounds> Backend<R> {
    #[inline(always)]
    /// This is essentially the same as the original `gen_par_ks_blocks` method except
    /// it takes a pointer instead.
    fn write_par_ks_blocks(&mut self, mut dest: *mut u8) {
        macro_rules! rotate_left {
            ($v:ident, 8) => {{
                let maskb = [3u8, 0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 10, 15, 12, 13, 14];
                let mask = vld1q_u8(maskb.as_ptr());

                vreinterpretq_u32_u8(vqtbl1q_u8(vreinterpretq_u8_u32($v), mask))
            }};
            ($v:ident, 16) => {
                vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32($v)))
            };
            ($v:ident, $r:literal) => {
                vorrq_u32(vshlq_n_u32($v, $r), vshrq_n_u32($v, 32 - $r))
            };
        }

        macro_rules! extract {
            ($v:ident, $s:literal) => {
                vextq_u32($v, $v, $s)
            };
        }

        unsafe {
            let ctrs = [
                vld1q_u32([1, 0, 0, 0].as_ptr()),
                vld1q_u32([2, 0, 0, 0].as_ptr()),
                vld1q_u32([3, 0, 0, 0].as_ptr()),
                vld1q_u32([4, 0, 0, 0].as_ptr()),
            ];

            let mut r0_0 = self.state[0];
            let mut r0_1 = self.state[1];
            let mut r0_2 = self.state[2];
            let mut r0_3 = self.state[3];

            let mut r1_0 = self.state[0];
            let mut r1_1 = self.state[1];
            let mut r1_2 = self.state[2];
            let mut r1_3 = add64!(r0_3, ctrs[0]);

            let mut r2_0 = self.state[0];
            let mut r2_1 = self.state[1];
            let mut r2_2 = self.state[2];
            let mut r2_3 = add64!(r0_3, ctrs[1]);

            let mut r3_0 = self.state[0];
            let mut r3_1 = self.state[1];
            let mut r3_2 = self.state[2];
            let mut r3_3 = add64!(r0_3, ctrs[2]);

            for _ in 0..R::COUNT {
                r0_0 = vaddq_u32(r0_0, r0_1);
                r1_0 = vaddq_u32(r1_0, r1_1);
                r2_0 = vaddq_u32(r2_0, r2_1);
                r3_0 = vaddq_u32(r3_0, r3_1);

                r0_3 = veorq_u32(r0_3, r0_0);
                r1_3 = veorq_u32(r1_3, r1_0);
                r2_3 = veorq_u32(r2_3, r2_0);
                r3_3 = veorq_u32(r3_3, r3_0);

                r0_3 = rotate_left!(r0_3, 16);
                r1_3 = rotate_left!(r1_3, 16);
                r2_3 = rotate_left!(r2_3, 16);
                r3_3 = rotate_left!(r3_3, 16);

                r0_2 = vaddq_u32(r0_2, r0_3);
                r1_2 = vaddq_u32(r1_2, r1_3);
                r2_2 = vaddq_u32(r2_2, r2_3);
                r3_2 = vaddq_u32(r3_2, r3_3);

                r0_1 = veorq_u32(r0_1, r0_2);
                r1_1 = veorq_u32(r1_1, r1_2);
                r2_1 = veorq_u32(r2_1, r2_2);
                r3_1 = veorq_u32(r3_1, r3_2);

                r0_1 = rotate_left!(r0_1, 12);
                r1_1 = rotate_left!(r1_1, 12);
                r2_1 = rotate_left!(r2_1, 12);
                r3_1 = rotate_left!(r3_1, 12);

                r0_0 = vaddq_u32(r0_0, r0_1);
                r1_0 = vaddq_u32(r1_0, r1_1);
                r2_0 = vaddq_u32(r2_0, r2_1);
                r3_0 = vaddq_u32(r3_0, r3_1);

                r0_3 = veorq_u32(r0_3, r0_0);
                r1_3 = veorq_u32(r1_3, r1_0);
                r2_3 = veorq_u32(r2_3, r2_0);
                r3_3 = veorq_u32(r3_3, r3_0);

                r0_3 = rotate_left!(r0_3, 8);
                r1_3 = rotate_left!(r1_3, 8);
                r2_3 = rotate_left!(r2_3, 8);
                r3_3 = rotate_left!(r3_3, 8);

                r0_2 = vaddq_u32(r0_2, r0_3);
                r1_2 = vaddq_u32(r1_2, r1_3);
                r2_2 = vaddq_u32(r2_2, r2_3);
                r3_2 = vaddq_u32(r3_2, r3_3);

                r0_1 = veorq_u32(r0_1, r0_2);
                r1_1 = veorq_u32(r1_1, r1_2);
                r2_1 = veorq_u32(r2_1, r2_2);
                r3_1 = veorq_u32(r3_1, r3_2);

                r0_1 = rotate_left!(r0_1, 7);
                r1_1 = rotate_left!(r1_1, 7);
                r2_1 = rotate_left!(r2_1, 7);
                r3_1 = rotate_left!(r3_1, 7);

                r0_1 = extract!(r0_1, 1);
                r0_2 = extract!(r0_2, 2);
                r0_3 = extract!(r0_3, 3);

                r1_1 = extract!(r1_1, 1);
                r1_2 = extract!(r1_2, 2);
                r1_3 = extract!(r1_3, 3);

                r2_1 = extract!(r2_1, 1);
                r2_2 = extract!(r2_2, 2);
                r2_3 = extract!(r2_3, 3);

                r3_1 = extract!(r3_1, 1);
                r3_2 = extract!(r3_2, 2);
                r3_3 = extract!(r3_3, 3);

                r0_0 = vaddq_u32(r0_0, r0_1);
                r1_0 = vaddq_u32(r1_0, r1_1);
                r2_0 = vaddq_u32(r2_0, r2_1);
                r3_0 = vaddq_u32(r3_0, r3_1);

                r0_3 = veorq_u32(r0_3, r0_0);
                r1_3 = veorq_u32(r1_3, r1_0);
                r2_3 = veorq_u32(r2_3, r2_0);
                r3_3 = veorq_u32(r3_3, r3_0);

                r0_3 = rotate_left!(r0_3, 16);
                r1_3 = rotate_left!(r1_3, 16);
                r2_3 = rotate_left!(r2_3, 16);
                r3_3 = rotate_left!(r3_3, 16);

                r0_2 = vaddq_u32(r0_2, r0_3);
                r1_2 = vaddq_u32(r1_2, r1_3);
                r2_2 = vaddq_u32(r2_2, r2_3);
                r3_2 = vaddq_u32(r3_2, r3_3);

                r0_1 = veorq_u32(r0_1, r0_2);
                r1_1 = veorq_u32(r1_1, r1_2);
                r2_1 = veorq_u32(r2_1, r2_2);
                r3_1 = veorq_u32(r3_1, r3_2);

                r0_1 = rotate_left!(r0_1, 12);
                r1_1 = rotate_left!(r1_1, 12);
                r2_1 = rotate_left!(r2_1, 12);
                r3_1 = rotate_left!(r3_1, 12);

                r0_0 = vaddq_u32(r0_0, r0_1);
                r1_0 = vaddq_u32(r1_0, r1_1);
                r2_0 = vaddq_u32(r2_0, r2_1);
                r3_0 = vaddq_u32(r3_0, r3_1);

                r0_3 = veorq_u32(r0_3, r0_0);
                r1_3 = veorq_u32(r1_3, r1_0);
                r2_3 = veorq_u32(r2_3, r2_0);
                r3_3 = veorq_u32(r3_3, r3_0);

                r0_3 = rotate_left!(r0_3, 8);
                r1_3 = rotate_left!(r1_3, 8);
                r2_3 = rotate_left!(r2_3, 8);
                r3_3 = rotate_left!(r3_3, 8);

                r0_2 = vaddq_u32(r0_2, r0_3);
                r1_2 = vaddq_u32(r1_2, r1_3);
                r2_2 = vaddq_u32(r2_2, r2_3);
                r3_2 = vaddq_u32(r3_2, r3_3);

                r0_1 = veorq_u32(r0_1, r0_2);
                r1_1 = veorq_u32(r1_1, r1_2);
                r2_1 = veorq_u32(r2_1, r2_2);
                r3_1 = veorq_u32(r3_1, r3_2);

                r0_1 = rotate_left!(r0_1, 7);
                r1_1 = rotate_left!(r1_1, 7);
                r2_1 = rotate_left!(r2_1, 7);
                r3_1 = rotate_left!(r3_1, 7);

                r0_1 = extract!(r0_1, 3);
                r0_2 = extract!(r0_2, 2);
                r0_3 = extract!(r0_3, 1);

                r1_1 = extract!(r1_1, 3);
                r1_2 = extract!(r1_2, 2);
                r1_3 = extract!(r1_3, 1);

                r2_1 = extract!(r2_1, 3);
                r2_2 = extract!(r2_2, 2);
                r2_3 = extract!(r2_3, 1);

                r3_1 = extract!(r3_1, 3);
                r3_2 = extract!(r3_2, 2);
                r3_3 = extract!(r3_3, 1);
            }

            r0_0 = vaddq_u32(r0_0, self.state[0]);
            r0_1 = vaddq_u32(r0_1, self.state[1]);
            r0_2 = vaddq_u32(r0_2, self.state[2]);
            r0_3 = vaddq_u32(r0_3, self.state[3]);

            r1_0 = vaddq_u32(r1_0, self.state[0]);
            r1_1 = vaddq_u32(r1_1, self.state[1]);
            r1_2 = vaddq_u32(r1_2, self.state[2]);
            r1_3 = vaddq_u32(r1_3, self.state[3]);
            r1_3 = add64!(r1_3, ctrs[0]);

            r2_0 = vaddq_u32(r2_0, self.state[0]);
            r2_1 = vaddq_u32(r2_1, self.state[1]);
            r2_2 = vaddq_u32(r2_2, self.state[2]);
            r2_3 = vaddq_u32(r2_3, self.state[3]);
            r2_3 = add64!(r2_3, ctrs[1]);

            r3_0 = vaddq_u32(r3_0, self.state[0]);
            r3_1 = vaddq_u32(r3_1, self.state[1]);
            r3_2 = vaddq_u32(r3_2, self.state[2]);
            r3_3 = vaddq_u32(r3_3, self.state[3]);
            r3_3 = add64!(r3_3, ctrs[2]);

            vst1q_u8(dest.offset(0), vreinterpretq_u8_u32(r0_0));
            vst1q_u8(dest.offset(16), vreinterpretq_u8_u32(r0_1));
            vst1q_u8(dest.offset(2 * 16), vreinterpretq_u8_u32(r0_2));
            vst1q_u8(dest.offset(3 * 16), vreinterpretq_u8_u32(r0_3));

            dest = dest.add(64);
            vst1q_u8(dest.offset(0), vreinterpretq_u8_u32(r1_0));
            vst1q_u8(dest.offset(16), vreinterpretq_u8_u32(r1_1));
            vst1q_u8(dest.offset(2 * 16), vreinterpretq_u8_u32(r1_2));
            vst1q_u8(dest.offset(3 * 16), vreinterpretq_u8_u32(r1_3));

            dest = dest.add(64);
            vst1q_u8(dest.offset(0), vreinterpretq_u8_u32(r2_0));
            vst1q_u8(dest.offset(16), vreinterpretq_u8_u32(r2_1));
            vst1q_u8(dest.offset(2 * 16), vreinterpretq_u8_u32(r2_2));
            vst1q_u8(dest.offset(3 * 16), vreinterpretq_u8_u32(r2_3));

            dest = dest.add(64);
            vst1q_u8(dest.offset(0), vreinterpretq_u8_u32(r3_0));
            vst1q_u8(dest.offset(16), vreinterpretq_u8_u32(r3_1));
            vst1q_u8(dest.offset(2 * 16), vreinterpretq_u8_u32(r3_2));
            vst1q_u8(dest.offset(3 * 16), vreinterpretq_u8_u32(r3_3));

            self.state[3] = add64!(self.state[3], ctrs[3]);
        }
    }
}
