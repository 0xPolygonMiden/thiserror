#[cfg(all(not(feature = "std"), error_in_core))]
use core::error::{Error, Request};
#[cfg(feature = "std")]
use std::error::{Error, Request};
#[cfg(not(any(feature = "std", error_in_core)))]
compile_error!("cannot compile this feature without the 'std' feature or a nightly compiler");

#[doc(hidden)]
pub trait ThiserrorProvide: Sealed {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>);
}

impl<T> ThiserrorProvide for T
where
    T: Error + ?Sized,
{
    #[inline]
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>) {
        self.provide(request);
    }
}

#[doc(hidden)]
pub trait Sealed {}
impl<T: Error + ?Sized> Sealed for T {}
