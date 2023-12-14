/// This file was made primarily because VariantCounter needs to be "public"
/// in order for Variant to compile. This way, we don't need to expose
/// VariantCounter when it is only for internal use.

/// A trait that distinguishes some ChaCha variants
pub trait Variant: Clone {
    /// the size of the Nonce in u32s
    const NONCE_INDEX: usize;
}

#[derive(Clone)]
/// The details pertaining to the IETF variant
pub struct Ietf();
impl Variant for Ietf {
    const NONCE_INDEX: usize = 13;
}

#[derive(Clone)]
#[cfg(feature = "legacy")]
pub struct Legacy();

#[cfg(feature = "legacy")]
impl Variant for Legacy {
    const NONCE_INDEX: usize = 14;
}

#[derive(Clone)]
#[cfg(feature = "xchacha")]
pub struct XChaChaVariant {}

#[cfg(feature = "xchacha")]
impl Variant for XChaChaVariant {
    const NONCE_INDEX: usize = 13;
}
