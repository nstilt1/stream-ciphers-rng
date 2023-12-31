use cfg_if::cfg_if;

#[cfg(feature = "cipher")]
use cipher::{consts::U64, StreamBackend, BlockSizeUser, ParBlocksSizeUser};

use crate::STATE_WORDS;

pub(crate) mod soft;

cfg_if! {
    if #[cfg(chacha20_force_soft)] {
        pub use self::soft::Backend;
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
        pub use self::neon::Backend;
    } else {
        pub(crate) mod soft;
        pub use self::soft::ChaChaCore;
    }
}

#[cfg(not(feature = "cipher"))]
pub(crate) trait CipherMethods: Sized {}

#[cfg(feature = "cipher")]
pub(crate) trait CipherMethods: StreamBackend + Sized + ParBlocksSizeUser + BlockSizeUser<BlockSize = U64> {}

pub(crate) trait BackendType {
    const PAR_BLOCKS: usize;

    fn new(state: &mut [u32; STATE_WORDS]) -> Self;

    fn get_block_pos(&self) -> u32;

    fn set_block_pos(&mut self, pos: u32);

    #[cfg(feature = "rand_core")]
    fn set_nonce(&mut self, nonce: [u32; 3]);

    #[cfg(feature = "rand_core")]
    fn get_nonce(&self) -> [u32; 3];

    #[cfg(feature = "rand_core")]
    fn get_seed(&self) -> [u32; 8];
    
    fn increment_counter(&mut self, amount: i32);

    unsafe fn write_ks_blocks(&mut self, dest_ptr: *mut u8, num_blocks: usize);

    #[cfg(feature = "rand_core")]
    /// Generates `num_blocks * 64` bytes and blindly writes them to `dest_ptr`
    /// 
    /// # Safety
    /// `dest_ptr` must have at least `num_blocks * 4` bytes available to be 
    /// overwritten, or else it could produce undefined behavior
    fn rng_inner(&mut self, mut dest_ptr: *mut u8, num_blocks: usize) {
        let num_chunks = num_blocks / Self::PAR_BLOCKS;
        let remaining = num_blocks % Self::PAR_BLOCKS;
        unsafe {
            for _chunk in 0..num_chunks {
                self.write_ks_blocks(dest_ptr, Self::PAR_BLOCKS);
                dest_ptr = dest_ptr.add(Self::PAR_BLOCKS * 64);
            }
            if remaining > 0 {
                self.write_ks_blocks(dest_ptr, remaining);
            }
        }
    }
}