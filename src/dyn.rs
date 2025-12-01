//! Dynamic (trait object) versions of the awaiter traits.
//!
//! This module provides type-erased versions of the awaiter traits that can be
//! used as trait objects (`dyn DynAwaiter`, etc.). These are useful when you need
//! to store awaiters in collections or pass them through APIs that cannot be generic.
//!
//! The main traits in this module are:
//!
//! - [`DynAwaiter`] - Dynamic version of [`crate::Awaiter`]
//! - [`DynAwaiterMut`] - Dynamic version of [`crate::AwaiterMut`]
//! - [`DynUnsafeAwaiter`] - Dynamic version of [`crate::UnsafeAwaiter`]
//! - [`DynUnsafeAwaiterMut`] - Dynamic version of [`crate::UnsafeAwaiterMut`]
//!
//! These traits only support futures with `()` output type for object safety,
//! but the implementations convert generic futures using a side-channel pattern.

use core::{mem::MaybeUninit, pin::pin};

use crate::*;

/// Dynamic trait for awaiting futures with shared access.
///
/// This is the object-safe version of [`Awaiter`], limited to `()` output
/// for trait object compatibility.
pub trait DynAwaiter: DynAwaiterMut + DynUnsafeAwaiter {
    /// Awaits a future that produces `()`.
    fn dyn_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}

/// Dynamic trait for awaiting futures with mutable access.
///
/// This is the object-safe version of [`AwaiterMut`], limited to `()` output
/// for trait object compatibility.
pub trait DynAwaiterMut: DynUnsafeAwaiterMut {
    /// Awaits a future that produces `()` with mutable access.
    fn dyn_await_mut(&mut self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}

/// Dynamic trait for unsafely awaiting futures with shared access.
///
/// # Safety
///
/// Same safety requirements as [`UnsafeAwaiter`].
pub trait DynUnsafeAwaiter: DynUnsafeAwaiterMut {
    /// Unsafely awaits a future that produces `()`.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn dyn_unsafe_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}

/// Dynamic trait for unsafely awaiting futures with mutable access.
///
/// This is the base trait in the dynamic awaiter hierarchy.
///
/// # Safety
///
/// Same safety requirements as [`UnsafeAwaiterMut`].
pub trait DynUnsafeAwaiterMut {
    /// Unsafely awaits a future that produces `()` with mutable access.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn dyn_unsafe_await_mut(&mut self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}
impl<T: UnsafeAwaiterMut + ?Sized> DynUnsafeAwaiterMut for T{
    unsafe fn dyn_unsafe_await_mut(&mut self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> () {
        unsafe{self.unsafe_await_mut(f)}
    }
}
impl<T: UnsafeAwaiter + ?Sized> DynUnsafeAwaiter for T{
    unsafe fn dyn_unsafe_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> () {
        unsafe{
            self.unsafe_await(f)
        }
    }
}
impl<T: AwaiterMut + ?Sized> DynAwaiterMut for T{
    fn dyn_await_mut(&mut self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> () {
        self.await_mut(f)
    }
}
impl<T: Awaiter + ?Sized> DynAwaiter for T{
    fn dyn_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> () {
        self.r#await(f)
    }
}
macro_rules! awaiters {
    ([$($x:tt)*]) => {
        impl<'a> UnsafeAwaiterMut for dyn DynUnsafeAwaiterMut $($x)* + 'a{
            unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await_mut(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiter for dyn DynUnsafeAwaiter $($x)* + 'a{
            unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiterMut for dyn DynUnsafeAwaiter $($x)* + 'a{
            unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> AwaiterMut for dyn DynAwaiterMut $($x)* + 'a{
            fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_await_mut(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiterMut for dyn DynAwaiterMut $($x)* + 'a{
            unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await_mut(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiter for dyn DynAwaiterMut $($x)* + 'a{
            unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> Awaiter for dyn DynAwaiter $($x)* + 'a{
            fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_await(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> AwaiterMut for dyn DynAwaiter $($x)* + 'a{
            fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_await_mut(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiterMut for dyn DynAwaiter $($x)* + 'a{
            unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await_mut(f);
                unsafe{v.assume_init()}
            }
        }
        impl<'a> UnsafeAwaiter for dyn DynAwaiter $($x)* + 'a{
            unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
                let mut v = MaybeUninit::uninit();
                let mut f = async{
                    let f = f.await;
                    unsafe{v.write(f)};
                };
                let f: Pin<&mut dyn Future<Output = ()>> = unsafe{
                    Pin::new_unchecked(&mut f)
                };
                self.dyn_unsafe_await(f);
                unsafe{v.assume_init()}
            }
        }
    };
}
awaiters!([]);
awaiters!([+ Send]);
awaiters!([+ Sync]);
awaiters!([+ Send + Sync]);