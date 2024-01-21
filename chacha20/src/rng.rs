// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt::Debug;

use rand_core::{CryptoRng, Error, RngCore, SeedableRng, block::{BlockRng, BlockRngCore}};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    variants::Ietf,
    ChaChaCore, R12, R20, R8,
};

// number of 32-bit words per ChaCha block (fixed by algorithm definition)
const BLOCK_WORDS: u8 = 16;

/// The seed for ChaCha20. Implements ZeroizeOnDrop when the
/// zeroize feature is enabled.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Seed([u8; 32]);

impl Default for Seed {
    fn default() -> Self {
        Self([0u8; 32])
    }
}
impl AsRef<[u8; 32]> for Seed {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}
impl AsMut<[u8]> for Seed {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl From<[u8; 32]> for Seed {
    #[cfg(feature = "zeroize")]
    fn from(mut value: [u8; 32]) -> Self {
        let input = Self(value);
        value.zeroize();
        input
    }
    #[cfg(not(feature = "zeroize"))]
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[cfg(feature = "zeroize")]
impl Drop for Seed {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
#[cfg(feature = "zeroize")]
impl ZeroizeOnDrop for Seed {}

impl Debug for Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// A wrapper for set_word_pos() input that can be initialized with:
/// * `u64`
/// * `[u8; 5]`
pub struct WordPosInput {
    block_pos: u32,
    index: u8,
}

impl From<[u8; 5]> for WordPosInput {
    #[inline]
    fn from(value: [u8; 5]) -> Self {
        Self {
            block_pos: u32::from_le_bytes(value[0..4].try_into().unwrap()),
            index: (value[4] & 0b1111),
        }
    }
}

impl From<u64> for WordPosInput {
    #[inline]
    fn from(value: u64) -> Self {
        Self {
            block_pos: u32::from_le_bytes((value >> 4).to_le_bytes()[0..4].try_into().unwrap()),
            index: (value.to_le_bytes()[0] & 0x0F),
        }
    }
}

/// A wrapper for the `stream_id`. It can be initialized with a:
/// * `[u32; 3]`
/// * `[u8; 12]`
/// * `u128`
pub struct StreamId([u32; 3]);

impl From<[u32; 3]> for StreamId {
    #[inline]
    fn from(value: [u32; 3]) -> Self {
        StreamId(value)
    }
}

impl From<[u8; 12]> for StreamId {
    #[inline]
    fn from(value: [u8; 12]) -> Self {
        let mut result = Self([0u32; 3]);
        for (n, chunk) in result.0.iter_mut().zip(value.chunks_exact(4)) {
            *n = u32::from_le_bytes(chunk.try_into().unwrap())
        }
        result
    }
}

impl From<u128> for StreamId {
    #[inline]
    fn from(value: u128) -> Self {
        let mut result = Self([0u32; 3]);
        for (n, chunk) in result.0.iter_mut().zip(value.to_le_bytes()[0..12].chunks_exact(4)) {
            *n = u32::from_le_bytes(chunk.try_into().unwrap())
        }
        result
    }
}

/// A wrapper for `block_pos`. It can be initialized with:
/// * u32
/// * [u8; 4]
pub struct BlockPos(u32);

impl From<u32> for BlockPos {
    fn from(value: u32) -> Self {
        Self(value.to_le())
    }
}

impl From<[u8; 4]> for BlockPos {
    fn from(value: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(value))
    }
}

use cfg_if::cfg_if;

// note: this "works" in the sense that whichever target it is compiled for
// will have the proper BUFFER_SIZE; however, if it is compiled for
// avx2 and then packaged in some software that ends up on a lot of computers,
// x86/x86_64 computers that don't have avx2 will have a buffer of [u32; 64]
//
// it will still function, but it is not optimal
cfg_if! {
    if #[cfg(chacha20_force_soft)] {
        const BUFFER_SIZE: usize = BLOCK_WORDS as usize;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        cfg_if! {
            if #[cfg(chacha20_force_avx2)] {
                const BUFFER_SIZE: usize = BLOCK_WORDS as usize * 4;
            } else if #[cfg(chacha20_force_sse2)] {
                const BUFFER_SIZE: usize = BLOCK_WORDS as usize;
            } else if #[cfg(target_feature = "avx2")] {
                const BUFFER_SIZE: usize = BLOCK_WORDS as usize * 4;
            } else if #[cfg(target_feature = "sse2")] {
                const BUFFER_SIZE: usize = BLOCK_WORDS as usize;
            } else {
                const BUFFER_SIZE: usize = BLOCK_WORDS as usize;
            }
        }
    } else if #[cfg(all(chacha20_force_neon, target_arch = "aarch64", target_feature = "neon"))] {
        const BUFFER_SIZE: usize = BLOCK_WORDS as usize * 4;
    } else {
        const BUFFER_SIZE: usize = BLOCK_WORDS as usize;
    }
}

/// The results buffer. Aligned by 16-byte boundaries to try to increase 
/// SIMD performance.
#[cfg_attr(feature = "zeroize", derive(Copy, Clone))]
#[repr(align(16))]
pub struct BlockRngResults([u32; BUFFER_SIZE]);

impl AsRef<[u32]> for BlockRngResults {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}

impl AsMut<[u32]> for BlockRngResults {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}

impl Default for BlockRngResults {
    fn default() -> Self {
        Self([0u32; BUFFER_SIZE])
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::DefaultIsZeroes for BlockRngResults {}

// NB. this must remain consistent with some currently hard-coded numbers in this module
const BUF_BLOCKS: u8 = BUFFER_SIZE as u8 >> 4;

macro_rules! impl_chacha_rng {
    ($ChaChaXRng:ident, $ChaChaXCore:ident, $rounds:ident, $abst: ident) => {
        /// A cryptographically secure random number generator that uses the ChaCha algorithm.
        ///
        /// ChaCha is a stream cipher designed by Daniel J. Bernstein[^1], that we use as an RNG. It is
        /// an improved variant of the Salsa20 cipher family, which was selected as one of the "stream
        /// ciphers suitable for widespread adoption" by eSTREAM[^2].
        ///
        /// ChaCha uses add-rotate-xor (ARX) operations as its basis. These are safe against timing
        /// attacks, although that is mostly a concern for ciphers and not for RNGs. We provide a SIMD
        /// implementation to support high throughput on a variety of common hardware platforms.
        ///
        /// With the ChaCha algorithm it is possible to choose the number of rounds the core algorithm
        /// should run. The number of rounds is a tradeoff between performance and security, where 8
        /// rounds is the minimum potentially secure configuration, and 20 rounds is widely used as a
        /// conservative choice.
        ///
        /// We use a 32-bit counter and 96-bit stream identifier as in the IETF implementation[^3]
        /// except that we use a stream identifier in place of a nonce. A 32-bit counter over 64-byte
        /// (16 word) blocks allows 256 GiB of output before cycling, and the stream identifier allows
        /// 2<sup>96</sup> unique streams of output per seed. Both counter and stream are initialized
        /// to zero but may be set via the `set_word_pos` and `set_stream` methods.
        ///
        /// The word layout is:
        ///
        /// ```text
        /// constant  constant  constant  constant
        /// seed      seed      seed      seed
        /// seed      seed      seed      seed
        /// counter   stream_id stream_id stream_id
        /// ```
        /// This implementation uses an output buffer of sixteen `u32` words.
        ///
        /// ```rust
        /// use chacha20::ChaCha20Rng;
        /// // use rand_core traits
        /// use rand_core::{SeedableRng, RngCore};
        ///
        /// let seed = [42u8; 32];
        /// let mut rng = ChaCha20Rng::from_seed(seed);
        /// 
        /// // `.set_stream()` can be used with a u128
        /// rng.set_stream(100);
        /// // or a [u8; 12]
        /// rng.set_stream([3u8; 12]);
        /// // or a [u32; 3]
        /// rng.set_stream([9u32; 3]);
        ///
        /// // `.set_word_pos()` can be used with a u64
        /// rng.set_word_pos(5);
        /// // or a [u8; 5]
        /// rng.set_word_pos([4u8; 5]);
        ///
        /// let x = rng.next_u32();
        /// let mut array = [0u8; 32];
        /// rng.fill_bytes(&mut array);
        /// 
        /// // if you need to clear the RNG's buffer, don't forget to call zeroize
        /// # #[cfg(feature = "zeroize")]
        /// use zeroize::Zeroize;
        /// # #[cfg(feature = "zeroize")]
        /// rng.zeroize();
        /// ```
        ///
        /// The other Rngs from this crate are initialized similarly.
        ///
        /// [^1]: D. J. Bernstein, [*ChaCha, a variant of Salsa20*](
        ///       https://cr.yp.to/chacha.html)
        ///
        /// [^2]: [eSTREAM: the ECRYPT Stream Cipher Project](
        ///       http://www.ecrypt.eu.org/stream/)
        ///
        /// [^3]: Internet Research Task Force, [*ChaCha20 and Poly1305 for IETF Protocols*](
        ///       https://www.rfc-editor.org/rfc/rfc8439)
        #[cfg_attr(docsrs, doc(cfg(feature = "rng")))]
        #[derive(Clone)]
        pub struct $ChaChaXRng {
            /// The ChaCha core random number generator
            rng: BlockRng<$ChaChaXCore>,
        }

        /// The ChaCha core random number generator
        #[derive(Clone)]
        pub struct $ChaChaXCore(ChaChaCore<$rounds, Ietf>);

        impl BlockRngCore for $ChaChaXCore {
            type Item = u32;
            type Results = BlockRngResults;
            #[inline]
            fn generate(&mut self, r: &mut Self::Results) {
                unsafe {
                    self.0.generate(r.0.as_mut_ptr() as *mut u8, BUF_BLOCKS as usize)
                }
            }
        }

        impl $ChaChaXCore {
            /// Sets the block pos. This does not affect the RNG's index, so if it has
            /// already changed, it will not reset it.
            ///
            /// This can be used with either:
            /// * u32
            /// * [u8; 4]
            pub fn set_block_pos<B: Into<BlockPos>>(&mut self, pos: B) {
                self.0.state[12] = pos.into().0;
                self.0.update_state()
            }

            /// Gets the block pos.
            pub fn get_block_pos(&self) -> u32 {
                self.0.state[12]
            }
        }

        impl SeedableRng for $ChaChaXRng {
            type Seed = [u8; 32];

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                Self {
                    rng: BlockRng::new($ChaChaXCore(ChaChaCore::<$rounds, Ietf>::from_seed(seed.into())))
                }
            }
        }

        impl RngCore for $ChaChaXRng {
            #[inline]
            fn next_u32(&mut self) -> u32 {
                self.rng.next_u32()
            }

            #[inline]
            fn next_u64(&mut self) -> u64 {
                self.rng.next_u64()
            }

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                self.rng.fill_bytes(dest)
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                self.rng.try_fill_bytes(dest)
            }
        }

        impl CryptoRng for $ChaChaXRng {}

        // Custom Debug implementation that does not expose the internal state
        impl Debug for $ChaChaXRng {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "ChaChaXCore {{}}")
            }
        }

        impl SeedableRng for ChaChaCore<$rounds, Ietf> {
            type Seed = Seed;

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                ChaChaCore::<$rounds, Ietf>::new(seed.as_ref(), &[0u8; 12])
            }
        }

        impl $ChaChaXRng {
            // The buffer is a 4-block window, i.e. it is always at a block-aligned position in the
            // stream but if the stream has been sought it may not be self-aligned.

            /// Get the offset from the start of the stream, in 32-bit words.
            ///
            /// Since the generated blocks are 64 words (2<sup>6</sup>) long and the
            /// counter is 32-bits, the offset is a 36-bit number. Sub-word offsets are
            /// not supported, hence the result can simply be multiplied by 4 to get a
            /// byte-offset.
            #[inline]
            pub fn get_word_pos(&self) -> u64 {
                let mut result =
                    u64::from(
                        self.rng.core.get_block_pos()
                            .wrapping_sub(BUF_BLOCKS.into())
                    ) << 4;

                result += self.rng.index() as u64;
                // eliminate bits above the 36th bit
                result & 0xfffffffff
            }

            /// Set the offset from the start of the stream, in 32-bit words. This method
            /// takes either:
            /// * u64
            /// * [u8; 5]
            ///
            /// As with `get_word_pos`, we use a 36-bit number. When given a `u64`, the
            /// least significant 4 bits are used for the RNG's index, and the 32 bits 
            /// before it are used to set the block position.
            /// 
            /// When given a `[u8; 5]`, the word_pos is set a little more arbitrarily.
            #[inline]
            pub fn set_word_pos<W: Into<WordPosInput>>(&mut self, word_offset: W) {
                let word_offset: WordPosInput = word_offset.into();
                self.rng.core.set_block_pos(word_offset.block_pos);

                if word_offset.index == 0 {
                    self.rng.reset()
                } else {
                    self.rng.generate_and_set(word_offset.index as usize)
                }
            }

            /// Set the stream number. The lower 96 bits are used and the rest are
            /// discarded. This method takes either:
            /// * [u32; 3]
            /// * [u8; 12]
            /// * u128
            ///
            /// This is initialized to zero; 2<sup>96</sup> unique streams of output
            /// are available per seed/key.
            #[inline]
            pub fn set_stream<S: Into<StreamId>>(&mut self, stream: S) {
                let stream: StreamId = stream.into();
                self.rng.core.0.set_nonce(stream.0);

                if self.rng.index() != BUFFER_SIZE {
                    self.rng.generate_and_set(self.rng.index());
                }
            }

            /// Get the stream number.
            #[inline]
            pub fn get_stream(&self) -> u128 {
                let mut result = [0u8; 16];
                for (i, &big) in self.rng.core.0.get_nonce()
                    .iter()
                    .enumerate()
                {
                    let index = i * 4;
                    result[index + 0] = big as u8;
                    result[index + 1] = (big >> 8) as u8;
                    result[index + 2] = (big >> 16) as u8;
                    result[index + 3] = (big >> 24) as u8;
                }
                u128::from_le_bytes(result)
            }

            /// Gets the current block position.
            #[inline]
            pub fn get_block_pos(&self) -> u32 {
                self.rng.core.get_block_pos()
            }

            /// Sets the block position and resets the RNG's index.
            /// 
            /// This method will accept either:
            /// - a `u32` or
            /// - a `[u8; 4]`
            #[inline]
            pub fn set_block_pos<B: Into<BlockPos>>(&mut self, pos: B) {
                self.rng.core.set_block_pos(pos.into().0);
                self.rng.reset()
            }

            /// Get the seed.
            #[inline]
            pub fn get_seed(&self) -> [u8; 32] {
                let mut result = [0u8; 32];
                for (i, &big) in self.rng.core.0.get_seed().iter().enumerate() {
                    let index = i * 4;
                    result[index + 0] = big as u8;
                    result[index + 1] = (big >> 8) as u8;
                    result[index + 2] = (big >> 16) as u8;
                    result[index + 3] = (big >> 24) as u8;
                }
                result
            }
        }

        #[cfg(feature = "zeroize")]
        impl Zeroize for $ChaChaXRng {
            fn zeroize(&mut self) {
                self.rng.zeroize();
            }
        }

        impl PartialEq<$ChaChaXRng> for $ChaChaXRng {
            fn eq(&self, rhs: &$ChaChaXRng) -> bool {
                let a: $abst::$ChaChaXRng = self.into();
                let b: $abst::$ChaChaXRng = rhs.into();
                a == b
            }
        }

        impl Eq for $ChaChaXRng {}

        #[cfg(feature = "serde1")]
        impl Serialize for $ChaChaXRng {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                $abst::$ChaChaXRng::from(self).serialize(s)
            }
        }
        #[cfg(feature = "serde1")]
        impl<'de> Deserialize<'de> for $ChaChaXRng {
            fn deserialize<D>(d: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                $abst::$ChaChaXRng::deserialize(d).map(|x| Self::from(&x))
            }
        }

        impl From<$ChaChaXCore> for $ChaChaXRng {
            fn from(core: $ChaChaXCore) -> Self {
                $ChaChaXRng {
                    rng: BlockRng::new(core)
                }
            }
        }

        mod $abst {
            #[cfg(feature = "serde1")]
            use serde::{Deserialize, Serialize};

            // The abstract state of a ChaCha stream, independent of implementation choices. The
            // comparison and serialization of this object is considered a semver-covered part of
            // the API.
            #[derive(Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
            pub(crate) struct $ChaChaXRng {
                seed: crate::rng::Seed,
                stream: u128,
                word_pos: u64,
            }

            impl From<&super::$ChaChaXRng> for $ChaChaXRng {
                // Forget all information about the input except what is necessary to determine the
                // outputs of any sequence of pub API calls.
                fn from(r: &super::$ChaChaXRng) -> Self {
                    Self {
                        seed: r.get_seed().into(),
                        stream: r.get_stream(),
                        word_pos: r.get_word_pos(),
                    }
                }
            }

            impl From<&$ChaChaXRng> for super::$ChaChaXRng {
                // Construct one of the possible concrete RNGs realizing an abstract state.
                fn from(a: &$ChaChaXRng) -> Self {
                    use rand_core::SeedableRng;
                    let mut r = Self::from_seed(a.seed.0.into());
                    r.set_stream(a.stream);
                    r.set_word_pos(a.word_pos);
                    r
                }
            }
        }
    };
}

impl_chacha_rng!(ChaCha8Rng, ChaCha8Core, R8, abst8);

impl_chacha_rng!(ChaCha12Rng, ChaCha12Core, R12, abst12);

impl_chacha_rng!(ChaCha20Rng, ChaCha20Core, R20, abst20);

#[cfg(test)]
mod tests {

    use super::*;
    use rand_core::{RngCore, SeedableRng};

    #[cfg(feature = "serde1")]
    use serde_json;

    const KEY: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_zeroize_inputs_internal() {
        let ptr = {
            let initial_seed: Seed = KEY.clone().into();
            initial_seed.0.as_ptr()
        };
        let memory_inspection = unsafe { core::slice::from_raw_parts(ptr, 32) };
        assert_ne!(&KEY, memory_inspection);
    }

    #[test]
    fn test_rng_output() {
        let mut rng = ChaCha20Rng::from_seed(KEY.into());
        let mut bytes = [0u8; 13];

        rng.fill_bytes(&mut bytes);
        assert_eq!(
            bytes,
            [177, 105, 126, 159, 198, 70, 30, 25, 131, 209, 49, 207, 105]
        );

        rng.fill_bytes(&mut bytes);
        assert_eq!(
            bytes,
            [167, 163, 252, 19, 79, 20, 152, 128, 232, 187, 43, 93, 35]
        );
    }

    #[test]
    fn test_wrapping_add() {
        let mut rng = ChaCha20Rng::from_entropy();
        rng.set_stream(1337 as u128);
        // test counter wrapping-add
        rng.set_word_pos((2 as u64).pow(36) - 1);
        let mut output = [3u8; 2048];
        rng.fill_bytes(&mut output);

        assert_ne!(output, [0u8; 2048]);

        if rng.get_word_pos() > 2000 {
            panic!("word pos fail; it is {:?}", rng.get_word_pos());
        }
        assert!(rng.get_word_pos() != 0);
    }

    #[test]
    fn test_set_and_get_equivalence() {
        let seed = [44u8; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);

        // test set_stream with [u32; 3]
        rng.set_stream([313453u32, 0u32, 0u32]);
        assert_eq!(rng.get_stream(), 313453);

        // test set_stream with [u8; 12]
        rng.set_stream([89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(rng.get_stream(), 89);

        // test set_stream with u128
        rng.set_stream(11111111);
        assert_eq!(rng.get_stream(), 11111111);

        // test set_block_pos with u32
        rng.rng.core.set_block_pos(58392);
        assert_eq!(rng.rng.core.get_block_pos(), 58392);

        // test set_block_pos with [u8; 4]
        rng.rng.core.set_block_pos([77, 0, 0, 0]);
        assert_eq!(rng.rng.core.get_block_pos(), 77);

        // test set_word_pos with u64
        rng.set_word_pos(8888);
        assert_eq!(rng.get_word_pos(), 8888);
    }

    #[test]
    /// Testing the edge cases of `fill_bytes()` by brute-forcing it with dest sizes
    /// that start at 1, and increase by 1 up to `N`, then they decrease from `N`
    /// to 1, and this can repeat multiple times if desired.
    ///
    /// This test uses `rand_chacha v0.3.1` because this version's API is directly
    /// based on `rand_chacha v0.3.1`, and previous versions of `chacha20` could be
    /// affected by rust flags for changing the backend. Also, it doesn't seem to work
    /// with `chacha20 v0.8`
    ///
    /// Because this test uses `rand_chacha v0.3.1` which uses a 64-bit counter, these
    /// test results should be accurate up to `block_pos = 2^32 - 1`.
    fn test_fill_bytes_v2() {
        use rand_chacha::ChaCha20Rng as TesterRng;
        use rand_chacha::rand_core::{RngCore, SeedableRng};

        let mut rng = ChaChaRng::from_seed([0u8; 32]);
        let mut tester_rng = TesterRng::from_seed([0u8; 32]);

        let stream_test: u64 = 34582934;
        // `chacha20`::ChaCha20Rng uses a 96-bit stream id and rand_chacha v0.3.1 uses a 64 bit stream id
        let mut chacha_stream_equivalent = [0u8; 12];
        chacha_stream_equivalent[4..].copy_from_slice(&stream_test.to_le_bytes());

        rng.set_stream(chacha_stream_equivalent);
        tester_rng.set_stream(stream_test);

        let word_pos_test: u32 = 33892;
        rng.set_word_pos(word_pos_test as u64);
        tester_rng.set_word_pos(word_pos_test.into());

        let num_iterations = 32;

        // If N is too large, it could cause stack overflow.
        // With N = 1445, the arrays are 1044735 bytes each, or 0.9963 MiB
        const N: usize = 1000;
        // compute the sum from 1 to N, with increments of 1
        const LEN: usize = (N * (N + 1)) / 2;

        let mut test_array: [u8; LEN];
        let mut tester_array: [u8; LEN];

        for _iteration in 0..num_iterations {
            test_array = [0u8; LEN];
            tester_array = [0u8; LEN];

            let mut dest_pos = 0;
            // test fill_bytes with lengths starting at 1 byte, increasing by 1,
            // up to N bytes
            for test_len in 1..=N {
                let debug_start_word_pos = rng.get_word_pos();
                let end_pos = dest_pos + test_len;

                // ensure that the current dest_pos index isn't overwritten already
                assert_eq!(test_array[dest_pos], 0);
                rng.fill_bytes(&mut test_array[dest_pos..end_pos]);
                tester_rng.fill_bytes(&mut tester_array[dest_pos..end_pos]);

                if test_array[dest_pos..end_pos] != tester_array[dest_pos..end_pos] {
                    for (t, (index, expected)) in test_array[dest_pos..end_pos]
                        .iter()
                        .zip(tester_array[dest_pos..end_pos].iter().enumerate())
                    {
                        if t != expected {
                            panic!(
                                "Failed test at start_word_pos = {},\nfailed index: {:?}\nFailing word_pos = {}\nComparing: {} == {}",
                                debug_start_word_pos,
                                index,
                                debug_start_word_pos + (index / 4) as u64,
                                t,
                                expected
                            );
                        }
                    }
                }
                assert_eq!(rng.next_u32(), tester_rng.next_u32());

                dest_pos = end_pos;
            }
            test_array = [0u8; LEN];
            tester_array = [0u8; LEN];
            dest_pos = 0;

            // test fill_bytes with lengths starting at N-1 bytes, decreasing by 1,
            // down to 1 byte
            for test_len in 1..=N {
                let debug_start_word_pos = rng.get_word_pos();
                let end_pos = dest_pos + N - test_len;

                // ensure that the current dest_pos index isn't overwritten already
                assert_eq!(test_array[dest_pos], 0);
                rng.fill_bytes(&mut test_array[dest_pos..end_pos]);
                tester_rng.fill_bytes(&mut tester_array[dest_pos..end_pos]);

                if test_array[dest_pos..end_pos] != tester_array[dest_pos..end_pos] {
                    for (t, (index, expected)) in test_array[dest_pos..end_pos]
                        .iter()
                        .zip(tester_array[dest_pos..end_pos].iter().enumerate())
                    {
                        if t != expected {
                            panic!(
                                "Failed test at start_word_pos = {},\nfailed index: {:?}\nFailing word_pos = {}\nComparing: {} == {}",
                                debug_start_word_pos,
                                index,
                                debug_start_word_pos + (index / 4) as u64,
                                t,
                                expected
                            );
                        }
                    }
                }
                assert_eq!(rng.next_u32(), tester_rng.next_u32());
                dest_pos = end_pos;
            }
        }
    }

    #[cfg(feature = "serde1")]
    use super::{ChaCha12Rng, ChaCha20Rng, ChaCha8Rng};

    type ChaChaRng = ChaCha20Rng;

    #[cfg(feature = "serde1")]
    #[test]
    fn test_chacha_serde_roundtrip() {
        let seed = [
            1, 0, 52, 0, 0, 0, 0, 0, 1, 0, 10, 0, 22, 32, 0, 0, 2, 0, 55, 49, 0, 11, 0, 0, 3, 0, 0,
            0, 0, 0, 2, 92,
        ];
        let mut rng1 = ChaCha20Rng::from_seed(seed);
        let mut rng2 = ChaCha12Rng::from_seed(seed);
        let mut rng3 = ChaCha8Rng::from_seed(seed);

        let encoded1 = serde_json::to_string(&rng1).unwrap();
        let encoded2 = serde_json::to_string(&rng2).unwrap();
        let encoded3 = serde_json::to_string(&rng3).unwrap();

        let mut decoded1: ChaCha20Rng = serde_json::from_str(&encoded1).unwrap();
        let mut decoded2: ChaCha12Rng = serde_json::from_str(&encoded2).unwrap();
        let mut decoded3: ChaCha8Rng = serde_json::from_str(&encoded3).unwrap();

        assert_eq!(rng1, decoded1);
        assert_eq!(rng2, decoded2);
        assert_eq!(rng3, decoded3);

        assert_eq!(rng1.next_u32(), decoded1.next_u32());
        assert_eq!(rng2.next_u32(), decoded2.next_u32());
        assert_eq!(rng3.next_u32(), decoded3.next_u32());
    }

    // This test validates that:
    // 1. a hard-coded serialization demonstrating the format at time of initial release can still
    //    be deserialized to a ChaChaRng
    // 2. re-serializing the resultant object produces exactly the original string
    //
    // Condition 2 is stronger than necessary: an equivalent serialization (e.g. with field order
    // permuted, or whitespace differences) would also be admissible, but would fail this test.
    // However testing for equivalence of serialized data is difficult, and there shouldn't be any
    // reason we need to violate the stronger-than-needed condition, e.g. by changing the field
    // definition order.
    #[cfg(feature = "serde1")]
    #[test]
    fn test_chacha_serde_format_stability() {
        let j = r#"{"seed":[4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8,15,16,23,42,4,8],"stream":27182818284,"word_pos":3141592653}"#;
        let r: ChaChaRng = serde_json::from_str(&j).unwrap();
        let j1 = serde_json::to_string(&r).unwrap();
        assert_eq!(j, j1);
    }

    #[test]
    fn test_chacha_construction() {
        let seed = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let mut rng1 = ChaChaRng::from_seed(seed);
        assert_eq!(rng1.next_u32(), 137206642);

        let mut rng2 = ChaChaRng::from_rng(rng1).unwrap();
        assert_eq!(rng2.next_u32(), 1325750369);
    }

    #[test]
    fn test_chacha_true_values_a() {
        // Test vectors 1 and 2 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(seed);

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0xade0b876, 0x903df1a0, 0xe56a5d40, 0x28bd8653, 0xb819d2bd, 0x1aed8da0, 0xccef36a8,
            0xc70d778b, 0x7c5941da, 0x8d485751, 0x3fe02477, 0x374ad8b8, 0xf4b8436a, 0x1ca11815,
            0x69b687c3, 0x8665eeb2,
        ];
        assert_eq!(results, expected);

        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0xbee7079f, 0x7a385155, 0x7c97ba98, 0x0d082d73, 0xa0290fcb, 0x6965e348, 0x3e53c612,
            0xed7aee32, 0x7621b729, 0x434ee69c, 0xb03371d5, 0xd539d874, 0x281fed31, 0x45fb0a51,
            0x1f0ae1ac, 0x6f4d794b,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_values_b() {
        // Test vector 3 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ];
        let mut rng = ChaChaRng::from_seed(seed);

        // Skip block 0
        for _ in 0..16 {
            rng.next_u32();
        }

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0x2452eb3a, 0x9249f8ec, 0x8d829d9b, 0xddd4ceb1, 0xe8252083, 0x60818b01, 0xf38422b8,
            0x5aaa49c9, 0xbb00ca8e, 0xda3ba7b4, 0xc4b592d1, 0xfdf2732f, 0x4436274e, 0x2561b3c8,
            0xebdd4aa6, 0xa0136c00,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_values_c() {
        // Test vector 4 from
        // https://tools.ietf.org/html/draft-nir-cfrg-chacha20-poly1305-04
        let seed = [
            0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        let expected = [
            0xfb4dd572, 0x4bc42ef1, 0xdf922636, 0x327f1394, 0xa78dea8f, 0x5e269039, 0xa1bebbc1,
            0xcaf09aae, 0xa25ab213, 0x48a6b46c, 0x1b9d9bcb, 0x092c5be6, 0x546ca624, 0x1bec45d5,
            0x87f47473, 0x96f0992e,
        ];
        let expected_end = 3 * 16;
        let mut results = [0u32; 16];

        // Test block 2 by skipping block 0 and 1
        let mut rng1 = ChaChaRng::from_seed(seed);
        // debugging word_pos
        assert_eq!(rng1.get_word_pos(), 0);
        for _ in 0..32 {
            rng1.next_u32();
        }
        for i in results.iter_mut() {
            *i = rng1.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng1.get_word_pos(), expected_end);

        // Test block 2 by using `set_word_pos`
        let mut rng2 = ChaChaRng::from_seed(seed);
        rng2.set_word_pos(2 * 16);
        for i in results.iter_mut() {
            *i = rng2.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng2.get_word_pos(), expected_end);

        // Test block 2 by using `set_block_pos` with a `u32`
        let mut rng3 = ChaChaRng::from_seed(seed);
        rng3.set_block_pos(2);
        results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng3.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng3.get_word_pos(), expected_end);

        // Test block 2 by using `set_block_pos` with a `[u8; 4]`
        let mut rng4 = ChaChaRng::from_seed(seed);
        rng4.set_block_pos([2, 0, 0, 0]);
        results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng4.next_u32();
        }
        assert_eq!(results, expected);
        assert_eq!(rng4.get_word_pos(), expected_end);

        // Test skipping behaviour with other types
        let mut buf = [0u8; 32];
        rng2.fill_bytes(&mut buf[..]);
        assert_eq!(rng2.get_word_pos(), expected_end + 8);
        rng2.fill_bytes(&mut buf[0..25]);
        assert_eq!(rng2.get_word_pos(), expected_end + 15);
        rng2.next_u64();
        assert_eq!(rng2.get_word_pos(), expected_end + 17);
        rng2.next_u32();
        rng2.next_u64();
        assert_eq!(rng2.get_word_pos(), expected_end + 20);
        rng2.fill_bytes(&mut buf[0..1]);
        assert_eq!(rng2.get_word_pos(), expected_end + 21);
    }

    #[test]
    fn test_chacha_multiple_blocks() {
        let seed = [
            0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7,
            0, 0, 0,
        ];
        let mut rng = ChaChaRng::from_seed(seed);

        // Store the 17*i-th 32-bit word,
        // i.e., the i-th word of the i-th 16-word block
        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
            for _ in 0..16 {
                rng.next_u32();
            }
        }
        let expected = [
            0xf225c81a, 0x6ab1be57, 0x04d42951, 0x70858036, 0x49884684, 0x64efec72, 0x4be2d186,
            0x3615b384, 0x11cfa18e, 0xd3c50049, 0x75c775f6, 0x434c6530, 0x2c5bad8f, 0x898881dc,
            0x5f1c86d9, 0xc1f8e7f4,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_true_bytes() {
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(seed);
        let mut results = [0u8; 32];
        rng.fill_bytes(&mut results);
        let expected = [
            118, 184, 224, 173, 160, 241, 61, 144, 64, 93, 106, 229, 83, 134, 189, 40, 189, 210,
            25, 184, 160, 141, 237, 26, 168, 54, 239, 204, 139, 119, 13, 199,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_nonce() {
        use hex_literal::hex;
        // Test vector 5 from
        // https://www.rfc-editor.org/rfc/rfc8439#section-2.3.2
        let seed = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
        let mut rng = ChaChaRng::from_seed(seed);

        let stream_id = hex!("000000090000004a00000000");
        rng.set_stream(stream_id);

        // The test vectors omit the first 64-bytes of the keystream
        let mut discard_first_64 = [0u8; 64];
        rng.fill_bytes(&mut discard_first_64);

        let mut results = [0u32; 16];
        for i in results.iter_mut() {
            *i = rng.next_u32();
        }
        let expected = [
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
            0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
            0xe883d0cb, 0x4e3c50a2,
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_chacha_clone_streams() {
        let seed = [
            0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7,
            0, 0, 0,
        ];
        let mut rng = ChaChaRng::from_seed(seed);
        let mut clone = rng.clone();
        for _ in 0..16 {
            assert_eq!(rng.next_u64(), clone.next_u64());
        }

        rng.set_stream(51);
        assert_eq!(rng.get_stream(), 51);
        assert_eq!(clone.get_stream(), 0);
        let mut fill_1 = [0u8; 7];
        rng.fill_bytes(&mut fill_1);
        let mut fill_2 = [0u8; 7];
        clone.fill_bytes(&mut fill_2);
        assert_ne!(fill_1, fill_2);
        for _ in 0..7 {
            assert!(rng.next_u64() != clone.next_u64());
        }
        clone.set_stream(51); // switch part way through block
        for _ in 7..16 {
            assert_eq!(rng.next_u64(), clone.next_u64());
        }
    }

    #[test]
    fn test_chacha_word_pos_wrap_exact() {
        use super::{BLOCK_WORDS, BUF_BLOCKS};
        let mut rng = ChaChaRng::from_seed(Default::default());
        // refilling the buffer in set_word_pos will wrap the block counter to 0
        let last_block = (2 as u64).pow(36) - u64::from(BUF_BLOCKS * BLOCK_WORDS);
        rng.set_word_pos(last_block);
        assert_eq!(rng.get_word_pos(), last_block);
    }

    #[test]
    fn test_chacha_word_pos_wrap_excess() {
        use super::BLOCK_WORDS;
        let mut rng = ChaChaRng::from_seed(Default::default());
        // refilling the buffer in set_word_pos will wrap the block counter past 0
        let last_block = (1 << 36) - u64::from(BLOCK_WORDS);
        rng.set_word_pos(last_block);
        assert_eq!(rng.get_word_pos(), last_block);
    }

    #[test]
    fn test_chacha_word_pos_zero() {
        let mut rng = ChaChaRng::from_seed(Default::default());
        assert_eq!(rng.get_word_pos(), 0);
        rng.set_word_pos(0);
        assert_eq!(rng.get_word_pos(), 0);
    }

    #[test]
    fn test_backend() {
        let seed: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                                13, 14, 15, 16, 17, 18, 19, 20, 21,
                                22, 23, 24, 25, 26, 27, 28, 29, 30,
                                31, 32];
        let mut rng = ChaChaRng::from_seed(seed);
        assert_eq!(seed, rng.get_seed());

        let stream_start = 5555555;
        let word_pos_start = 8888888;
        let test_amt = 100;

        for stream in stream_start..stream_start+test_amt {
            for block_pos in word_pos_start..word_pos_start+test_amt {
                rng.set_stream(stream as u128);
                assert_eq!(stream as u128, rng.get_stream());
                //rng.core.backend.set_block_pos(block_pos);
                //assert_eq!(rng.core.backend.get_block_pos(), block_pos);

                rng.set_word_pos(block_pos as u64);
                assert_eq!(rng.get_word_pos(), block_pos as u64);
            }
        }
    }

    // test for next rand_core version
    // #[test]
    // fn test_trait_objects() {
    //     use rand_core::CryptoRng;

    //     let mut rng1 = ChaChaRng::from_seed(Default::default());
    //     let rng2 = &mut rng1.clone() as &mut dyn CryptoRng;
    //     for _ in 0..1000 {
    //         assert_eq!(rng1.next_u64(), rng2.next_u64());
    //     }
    // }
}
