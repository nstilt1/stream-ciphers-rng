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
    type CounterVals: AsRef<[u32]>;
    /// A helper method for getting the block pos using these types
    fn into_block_counter(vals: &[u32]) -> Self::Counter;
    /// A helper method for setting the block pos using these types
    fn from_block_counter(val: Self::Counter) -> Self::CounterVals;
    /// A helper method for calculating the remaining blocks using these types
    fn remaining_blocks(block_pos: Self::Counter) -> Self::Counter;
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

    fn into_block_counter(vals: &[u32]) -> Self::Counter {
        vals[0]
    }
    fn from_block_counter(val: Self::Counter) -> Self::CounterVals {
        [val]
    }
    fn remaining_blocks(block_pos: Self::Counter) -> Self::Counter {
        u32::MAX - block_pos
    }
}

#[derive(Clone)]
#[cfg(feature = "legacy")]
pub struct Legacy();

#[cfg(feature = "legacy")]
impl VariantCounter for u64 {}

#[cfg(feature = "legacy")]
impl Variant for Legacy {
    type Counter = u32;
    const IS_U32: bool = true;
    type Nonce = crate::legacy::LegacyNonce;
    const NONCE_INDEX: usize = 14;
    type CounterVals = [u32; 1];
    fn into_block_counter(vals: &[u32]) -> Self::Counter {
        vals[0]
        //(vals[0] as u64) << 32 | (vals[1] as u64)
    }
    fn from_block_counter(val: Self::Counter) -> Self::CounterVals {
        [val]
        //[(val >> 32) as u32, val as u32];
    }
    fn remaining_blocks(block_counter: Self::Counter) -> Self::Counter {
        u32::MAX - block_counter
        //u64::MAX - block_counter
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
    const NONCE_INDEX: usize = 14;
    type CounterVals = [u32; 1];
    fn into_block_counter(vals: &[u32]) -> Self::Counter {
        vals[0]
    }
    fn from_block_counter(val: Self::Counter) -> Self::CounterVals {
        [val]
    }
    fn remaining_blocks(block_pos: Self::Counter) -> Self::Counter {
        u32::MAX - block_pos
    }
}
