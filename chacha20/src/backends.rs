use cfg_if::cfg_if;

#[cfg(feature = "cipher")]
use cipher::{consts::U64, StreamBackend, BlockSizeUser, ParBlocksSizeUser};

use crate::{STATE_WORDS, Rounds, Variant};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

pub(crate) mod soft;

cfg_if! {
    if #[cfg(chacha20_force_soft)] {
        pub use self::soft::ChaChaCore;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        // cfg_if! {
        //     if #[cfg(chacha20_force_avx2)] {
        //         pub(crate) mod avx2;
        //     } else if #[cfg(chacha20_force_sse2)] {
        //         pub(crate) mod sse2;
        //     } else {
        //         pub(crate) mod soft;
        //         pub(crate) mod avx2;
        //         pub(crate) mod sse2;
        //     }
        // }
        pub(crate) mod autodetect;
        pub(crate) mod avx2;
        pub(crate) mod sse2;

        pub(crate) use self::autodetect::ChaChaCore;
    } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        pub(crate) mod neon;
        pub use self::neon::ChaChaCore;
    } else {
        pub use self::soft::ChaChaCore;
    }
}

#[cfg(not(feature = "cipher"))]
pub(crate) trait CipherMethods: Sized {}

#[cfg(feature = "cipher")]
pub(crate) trait CipherMethods: StreamBackend + Sized + ParBlocksSizeUser + BlockSizeUser<BlockSize = U64> {}

pub(crate) trait BackendType {
    const PAR_BLOCKS: usize;

    fn new(state: &[u32; STATE_WORDS]) -> Self;

    fn update_state(&mut self, state: &[u32]);
    
    fn increment_counter(&mut self, amount: i32);

    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize);

    #[cfg(feature = "rng")]
    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    /// 
    /// # Safety
    /// `dest_ptr` must have at least `num_blocks * 4` bytes available to be 
    /// overwritten, or else it could produce undefined behavior
    fn rng_inner(&mut self, dest_ptr: *mut u8, num_blocks: usize) {
        unsafe {
            self.write_ks_blocks(dest_ptr, num_blocks);
        }
    }
}


#[cfg(feature = "cipher")]
impl<R: Rounds, V: Variant> BlockSizeUser for ChaChaCore<R, V> {
    type BlockSize = U64;
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> Drop for ChaChaCore<R, V> {
    fn drop(&mut self) {
        self.state.zeroize();
    }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<R: Rounds, V: Variant> ZeroizeOnDrop for ChaChaCore<R, V> {}