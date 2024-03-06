use crate::StdError;
use core::panic::UnwindSafe;

#[doc(hidden)]
pub trait AsDynError<'a>: Sealed {
    fn as_dyn_error(&self) -> &(dyn StdError + 'a);
}

impl<'a, T: StdError + 'a> AsDynError<'a> for T {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn StdError + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn StdError + 'a {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn StdError + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn StdError + Send + 'a {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn StdError + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn StdError + Send + Sync + 'a {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn StdError + 'a) {
        self
    }
}

impl<'a> AsDynError<'a> for dyn StdError + Send + Sync + UnwindSafe + 'a {
    #[inline]
    fn as_dyn_error(&self) -> &(dyn StdError + 'a) {
        self
    }
}

#[doc(hidden)]
pub trait Sealed {}
impl<'a, T: StdError + 'a> Sealed for T {}
impl<'a> Sealed for dyn StdError + 'a {}
impl<'a> Sealed for dyn StdError + Send + 'a {}
impl<'a> Sealed for dyn StdError + Send + Sync + 'a {}
impl<'a> Sealed for dyn StdError + Send + Sync + UnwindSafe + 'a {}
