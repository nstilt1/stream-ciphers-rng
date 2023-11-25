//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use crate::{ChaChaCore, Rounds, STATE_WORDS};

pub(crate) struct Backend<'a, R: Rounds>(pub(crate) &'a mut ChaChaCore<R>);

impl<'a, R: Rounds> Backend<'a, R> {
    #[inline(always)]
    pub(crate) fn gen_ks_blocks(&mut self) {
        for i in 0..4 {
            let res = run_rounds::<R>(&self.0.state);
            self.0.state[12] = self.0.state[12].wrapping_add(1);

            for (chunk, val) in self.0.buffer[i << 6..(i+1) << 6].chunks_exact_mut(4).zip(res.iter()) {
                chunk.copy_from_slice(&val.to_le_bytes());
            }
        }
    }
}

#[inline(always)]
fn run_rounds<R: Rounds>(state: &[u32; STATE_WORDS]) -> [u32; STATE_WORDS] {
    let mut res = *state;

    for _ in 0..R::COUNT {
        // column rounds
        quarter_round(0, 4, 8, 12, &mut res);
        quarter_round(1, 5, 9, 13, &mut res);
        quarter_round(2, 6, 10, 14, &mut res);
        quarter_round(3, 7, 11, 15, &mut res);

        // diagonal rounds
        quarter_round(0, 5, 10, 15, &mut res);
        quarter_round(1, 6, 11, 12, &mut res);
        quarter_round(2, 7, 8, 13, &mut res);
        quarter_round(3, 4, 9, 14, &mut res);
    }

    for (s1, s0) in res.iter_mut().zip(state.iter()) {
        *s1 = s1.wrapping_add(*s0);
    }
    res
}

/// The ChaCha20 quarter round function
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
