/// This file was made primarily because VariantCounter needs to be "public"
/// in order for Variant to compile. This way, we don't need to expose
/// VariantCounter when it is only for internal use.

#[cfg(feature = "cipher")]
/// A trait to restrict the counter for the cipher crate
pub trait VariantCounter: cipher::Counter {}
#[cfg(not(feature = "cipher"))]
pub trait VariantCounter {}

/// A trait that distinguishes some ChaCha variants
pub trait Variant: Clone {
    /// the type used for the variant's nonce
    type Nonce: AsRef<[u8]>;
    /// the size of the Nonce in u32s
    const NONCE_INDEX: usize;
    /// the counter's type
    type Counter: VariantCounter;
    /// a bool to help with figuring out the size of the Counter
    const IS_U32: bool;
    /// The result type of `from_block_counter` to handle either 1 or 2 u32 vals
    type CounterVals: core::ops::Index<usize, Output = u32>;
    /// Takes a &[u32; 2] and converts it into the Self::Counter type.
    /// The input is [state[12], state[13]]
    fn get_pos_helper(vals: &[u32]) -> Self::Counter;
    /// Breaks down the Self::Counter type into a u32 array for setting the 
    /// block pos.
    fn set_pos_helper(val: Self::Counter) -> Self::CounterVals;
    /// A helper method for calculating the remaining blocks using these types
    fn remaining_blocks(block_pos: Self::Counter) -> Option<usize>;
}

impl VariantCounter for u32 {}

#[derive(Clone)]
/// The details pertaining to the IETF variant
pub struct Ietf();
impl Variant for Ietf {
    type Counter = u32;
    const IS_U32: bool = true;
    type Nonce = [u8; 12];
    const NONCE_INDEX: usize = 13;
    type CounterVals = [u32; 1];

    fn get_pos_helper(vals: &[u32]) -> Self::Counter {
        vals[0]
    }
    fn set_pos_helper(val: Self::Counter) -> Self::CounterVals {
        [val]
    }
    fn remaining_blocks(block_pos: Self::Counter) -> Option<usize> {
        (u32::MAX - block_pos).try_into().ok()
    }
}

#[derive(Clone)]
#[cfg(feature = "legacy")]
pub struct Legacy();

#[cfg(feature = "legacy")]
impl VariantCounter for u64 {}

#[cfg(feature = "legacy")]
impl Variant for Legacy {
    type Counter = u64;
    const IS_U32: bool = false;
    type Nonce = crate::legacy::LegacyNonce;
    const NONCE_INDEX: usize = 14;
    type CounterVals = [u32; 2];

    /// Takes a &[u32; 2] and converts it into the Self::Counter type.
    /// The input is [state[12], state[13]]
    fn get_pos_helper(vals: &[u32]) -> Self::Counter {
        (vals[0] as u64) << 32 | (vals[1] as u64)
    }
    /// Breaks down the Self::Counter type into a u32 array for setting the 
    /// block pos.
    fn set_pos_helper(u64: Self::Counter) -> Self::CounterVals {
        [(u64 >> 32) as u32, u64 as u32]
    }
    fn remaining_blocks(block_counter: Self::Counter) -> Option<usize> {
        let remaining = u64::MAX - block_counter;
        // handle overflowing amount on 32-bit machines based on `stream_core.rs`
        #[cfg(target_pointer_width = "32")]
        if remaining > usize::MAX as u64 {
            None
        }
        remaining.try_into().ok()
    }
}

#[derive(Clone)]
#[cfg(feature = "xchacha")]
pub struct XChaChaVariant {}

#[cfg(feature = "xchacha")]
impl Variant for XChaChaVariant {
    type Counter = u32;
    type Nonce = [u8; 12];
    const IS_U32: bool = true;
    const NONCE_INDEX: usize = 13;
    type CounterVals = [u32; 1];
    fn get_pos_helper(vals: &[u32]) -> Self::Counter {
        vals[0]
    }
    fn set_pos_helper(val: Self::Counter) -> Self::CounterVals {
        [val]
    }
    fn remaining_blocks(block_pos: Self::Counter) -> Option<usize> {
        (u32::MAX - block_pos).try_into().ok()
    }
}
