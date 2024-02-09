use cfg_if::cfg_if;

#[cfg(feature = "cipher")]
use cipher::{consts::U64, BlockSizeUser};

use crate::{Rounds, Variant};

cfg_if! {
    if #[cfg(chacha20_force_soft)] {
        pub(crate) mod soft;
        pub use self::soft::ChaChaCore;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        pub(crate) mod autodetect;
        pub(crate) mod avx2;
        pub(crate) mod sse2;
        pub(crate) mod soft;

        pub(crate) use self::autodetect::ChaChaCore;
    } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        pub(crate) mod neon;
        pub use self::neon::ChaChaCore;
    } else {
        pub(crate) mod soft;
        pub use self::soft::ChaChaCore;
    }
}

/// Implements some common methods that the various `ChaChaCore`s use.
#[macro_export]
macro_rules! impl_chacha_core {
    () => {
        impl<R: Rounds, V: Variant> ChaChaCore<R, V> {
            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn set_nonce(&mut self, nonce: [u32; 3]) {
                self.state[13..16].copy_from_slice(&nonce);
                self.update_state();
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn get_nonce(&self) -> [u32; 3] {
                let mut result = [0u32; 3];
                result.copy_from_slice(&self.state[13..16]);
                result
            }

            #[inline]
            #[cfg(feature = "rng")]
            pub(crate) fn get_seed(&self) -> [u32; 8] {
                let mut result = [0u32; 8];
                result.copy_from_slice(&self.state[4..12]);
                result
            }
        }

        #[cfg(feature = "cipher")]
        impl<R: Rounds, V: Variant> StreamCipherSeekCore for ChaChaCore<R, V> {
            type Counter = u32;

            #[inline(always)]
            fn get_block_pos(&self) -> Self::Counter {
                self.state[12]
            }

            #[inline(always)]
            fn set_block_pos(&mut self, pos: Self::Counter) {
                self.state[12] = pos;
                self.update_state();
            }
        }
    };
}

#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for ChaChaCore<R, V> {
    type BlockSize = U64;
}
