//! # awaiter-trait
//!
//! A `no_std` compatible library providing traits for blocking on futures.
//!
//! This crate enables synchronous code to interact with asynchronous operations
//! by defining a hierarchy of awaiter and coroutine traits.
//!
//! ## Core Traits
//!
//! The awaiter traits form a hierarchy based on safety and mutability:
//!
//! - [`Awaiter`] - Safe awaiting with shared access (`&self`)
//! - [`AwaiterMut`] - Safe awaiting with mutable access (`&mut self`)
//! - [`UnsafeAwaiter`] - Unsafe awaiting with shared access
//! - [`UnsafeAwaiterMut`] - Unsafe awaiting with mutable access
//!
//! The coroutine traits enable executing synchronous code within async contexts:
//!
//! - [`Coroutine`] - Execute with a shared awaiter reference
//! - [`CoroutineMut`] - Execute with a mutable awaiter reference
//! - And their unsafe counterparts
//!
//! ## Features
//!
//! - **`embedded-io`** - Integration with `embedded-io` and `embedded-io-async` crates

#![no_std]

use core::pin::Pin;
pub mod r#dyn;
use r#dyn::*;
#[cfg(feature = "embedded-io")]
pub mod io;

/// A trait for synchronously awaiting futures with shared access.
///
/// This is the primary trait for blocking on futures when you have
/// shared (`&self`) access to the awaiter. Implementors must also
/// implement [`AwaiterMut`] and [`UnsafeAwaiter`].
///
/// # Example
///
/// ```ignore
/// use awaiter_trait::Awaiter;
/// use core::pin::Pin;
/// use core::future::Future;
///
/// fn use_awaiter(awaiter: &impl Awaiter) {
///     let fut = async { 42 };
///     let result = awaiter.r#await(core::pin::pin!(fut));
/// }
/// ```
pub trait Awaiter: AwaiterMut + UnsafeAwaiter {
    /// Blocks on a future until it completes, returning the output.
    ///
    /// # Parameters
    ///
    /// - `f`: A pinned mutable reference to the future to await
    ///
    /// # Returns
    ///
    /// The output value of the future once it completes
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}

/// A trait for synchronously awaiting futures with mutable access.
///
/// Similar to [`Awaiter`], but requires mutable access to the awaiter.
/// This is useful when the awaiter needs to modify internal state during awaiting.
pub trait AwaiterMut: UnsafeAwaiterMut {
    /// Blocks on a future with mutable access to self.
    fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}

/// A trait for unsafely awaiting futures with shared access.
///
/// # Safety
///
/// Implementations must ensure that calling `unsafe_await` is only done
/// in contexts where the safety invariants of the implementation are upheld.
pub trait UnsafeAwaiter: UnsafeAwaiterMut {
    /// Unsafely blocks on a future with shared access.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the implementation's safety requirements are met.
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}

/// A trait for unsafely awaiting futures with mutable access.
///
/// # Safety
///
/// Implementations must ensure that calling `unsafe_await_mut` is only done
/// in contexts where the safety invariants of the implementation are upheld.
pub trait UnsafeAwaiterMut {
    /// Unsafely blocks on a future with mutable access.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the implementation's safety requirements are met.
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}
#[doc(hidden)]
pub mod __ {
    pub use core;
}

/// A macro to automatically implement related awaiter and coroutine traits.
///
/// This macro reduces boilerplate by implementing the related traits in a trait
/// hierarchy. For example, if you implement [`Awaiter`], you can use this macro
/// to automatically implement [`AwaiterMut`], [`UnsafeAwaiter`], and [`UnsafeAwaiterMut`].
///
/// # Usage
///
/// ```ignore
/// use awaiter_trait::{Awaiter, autoimpl};
///
/// struct MyAwaiter;
///
/// impl Awaiter for MyAwaiter {
///     fn r#await<T>(&self, f: core::pin::Pin<&mut (dyn core::future::Future<Output = T> + '_)>) -> T {
///         // implementation
///     }
/// }
///
/// // Automatically implement AwaiterMut, UnsafeAwaiter, UnsafeAwaiterMut
/// autoimpl!(<> MyAwaiter as Awaiter);
/// ```
///
/// # Supported Trait Hierarchies
///
/// - `as Awaiter` - Implements `AwaiterMut`, `UnsafeAwaiter`, `UnsafeAwaiterMut`
/// - `as AwaiterMut` - Implements `UnsafeAwaiterMut`
/// - `as UnsafeAwaiter` - Implements `UnsafeAwaiterMut`
/// - `as Coroutine` - Implements all related coroutine traits
/// - `as CoroutineMut` - Implements unsafe variants
/// - And more...
#[macro_export]
macro_rules! autoimpl {
    (<$($g:ident $(: $b:path)? ),*> $t:ty as Awaiter) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::AwaiterMut for $t{
                fn await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            impl<$($g $(: $b)? ),*> $crate::UnsafeAwaiter for $t{
                unsafe fn unsafe_await<T>(&self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            $crate::autoimpl!(<$($g $(: $b)? ),*> $t as AwaiterMut);

        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as UnsafeAwaiter) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeAwaiterMut for $t{
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    unsafe{
                        <Self as $crate::UnsafeAwaiter>::unsafe_await(self,f)
                    }
                }
            }
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as AwaiterMut) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeAwaiterMut for $t{
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                   <Self as $crate::AwaiterMut>::await_mut(self,f)
                }
            }
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as UnsafeCoroutineMut) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineMutSelfMut for $t{
                unsafe fn unsafe_exec_mut_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynUnsafeAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::UnsafeCoroutineMut>::unsafe_exec_mut(self,move|a|f(a))
                    }
                }
            }
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as UnsafeCoroutineSelfMut) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineMutSelfMut for $t{
                unsafe fn unsafe_exec_mut_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynUnsafeAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::UnsafeCoroutineSelfMut>::unsafe_exec_self_mut(self,move|mut a|f(&mut a))
                    }
                }
            }
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as CoroutineMutSelfMut) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineMutSelfMut for $t{
                unsafe fn unsafe_exec_mut_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynUnsafeAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::CoroutineMutSelfMut>::exec_mut_self_mut(self,move|a|f(a))
                    }
                }
            }
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as UnsafeCoroutine) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineMut for $t{
                unsafe fn unsafe_exec_mut<T>(
                    &self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynUnsafeAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::UnsafeCoroutine>::unsafe_exec(self,move|a|f(a))
                    }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineSelfMut for $t{
                unsafe fn unsafe_exec_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(& (dyn $crate::r#dyn::DynUnsafeAwaiter + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::UnsafeCoroutine>::unsafe_exec(self,move|a|f(a))
                    }
                }
            }
            $crate::autoimpl!(<$($g $(: $b)? ),*> $t as UnsafeCoroutineMut);
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as CoroutineMut) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineMut for $t{
                unsafe fn unsafe_exec_mut<T>(
                    &self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynUnsafeAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::CoroutineMut>::exec_mut(self,move|a|f(a))
                    }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::CoroutineMutSelfMut for $t{
                fn exec_mut_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    // unsafe{
                        <Self as $crate::CoroutineMut>::exec_mut(self,move|a|f(a))
                    // }
                }
            }
            $crate::autoimpl!(<$($g $(: $b)? ),*> $t as UnsafeCoroutineMut);
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as CoroutineSelfMut) => {
        const _: () = {

            impl<$($g $(: $b)? ),*> $crate::CoroutineMutSelfMut for $t{
                fn exec_mut_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    // unsafe{
                        <Self as $crate::CoroutineSelfMut>::exec_self_mut(self,move|a|f(a))
                    // }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineSelfMut for $t{
                unsafe fn unsafe_exec_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(& (dyn $crate::r#dyn::DynUnsafeAwaiter + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::CoroutineSelfMut>::exec_self_mut(self,move|a|f(a))
                    }
                }
            }
            $crate::autoimpl!(<$($g $(: $b)? ),*> $t as CoroutineMutSelfMut);
        };
    };
    (<$($g:ident $(: $b:path)? ),*> $t:ty as Coroutine) => {
        const _: () = {
            impl<$($g $(: $b)? ),*> $crate::CoroutineMut for $t{
                fn exec_mut<T>(
                    &self,
                    f: impl FnOnce(&mut (dyn $crate::r#dyn::DynAwaiterMut + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::Coroutine>::exec(self,move|mut a|f(&mut a))
                    }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::CoroutineSelfMut for $t{
                fn exec_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(& (dyn $crate::r#dyn::DynAwaiter + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    // unsafe{
                        <Self as $crate::Coroutine>::exec(self,move|a|f(a))
                    // }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutine for $t{
                unsafe fn unsafe_exec<T>(
                    &self,
                    f: impl FnOnce(& (dyn $crate::r#dyn::DynUnsafeAwaiter + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::Coroutine>::exec(self,move|a|f(a))
                    }
                }
            }
            impl<$($g $(: $b)? ),*> $crate::UnsafeCoroutineSelfMut for $t{
                unsafe fn unsafe_exec_self_mut<T>(
                    &mut self,
                    f: impl FnOnce(& (dyn $crate::r#dyn::DynUnsafeAwaiter + '_)) -> T,
                ) -> impl $crate::__::core::future::Future<Output = T>{
                    unsafe{
                        <Self as $crate::UnsafeCoroutine>::unsafe_exec(self,move|a|f(a))
                    }
                }
            }
            $crate::autoimpl!(<$($g $(: $b)? ),*> $t as CoroutineMut);
            // $crate::autoimpl!(<$($g $(: $b)? ),*> $t as UnsafeCoroutineSelfMut);
        };
    };
}
impl<'a, T2: Awaiter + ?Sized> Awaiter for &'a T2 {
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        (&**self).r#await(f)
    }
}
impl<'a, T2: Awaiter + ?Sized> Awaiter for &'a mut T2 {
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        (&**self).r#await(f)
    }
}
impl<'a, T2: Awaiter + ?Sized> AwaiterMut for &'a T2 {
    fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        (&**self).r#await(f)
    }
}
impl<'a, T2: AwaiterMut + ?Sized> AwaiterMut for &'a mut T2 {
    fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        (&mut **self).await_mut(f)
    }
}
impl<'a, T2: UnsafeAwaiter + ?Sized> UnsafeAwaiter for &'a T2 {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<'a, T2: UnsafeAwaiter + ?Sized> UnsafeAwaiter for &'a mut T2 {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<'a, T2: UnsafeAwaiter + ?Sized> UnsafeAwaiterMut for &'a T2 {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<'a, T2: UnsafeAwaiterMut + ?Sized> UnsafeAwaiterMut for &'a mut T2 {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&mut **self).unsafe_await_mut(f) }
    }
}
impl<T2: UnsafeAwaiter + ?Sized> UnsafeAwaiter for *const T2 {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<T2: UnsafeAwaiter + ?Sized> UnsafeAwaiter for *mut T2 {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<T2: UnsafeAwaiter + ?Sized> UnsafeAwaiterMut for *const T2 {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&**self).unsafe_await(f) }
    }
}
impl<T2: UnsafeAwaiterMut + ?Sized> UnsafeAwaiterMut for *mut T2 {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        unsafe { (&mut **self).unsafe_await_mut(f) }
    }
}

/// A trait for unsafely executing synchronous code with mutable awaiter access.
///
/// This trait allows executing a closure that receives a mutable reference to
/// an awaiter, enabling synchronous code to block on futures within an async context.
///
/// # Safety
///
/// Implementations must ensure the awaiter is valid for the duration of the closure.
pub trait UnsafeCoroutineMut: UnsafeCoroutineMutSelfMut {
    /// Executes a closure with a mutable awaiter reference.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for safely executing synchronous code with mutable awaiter access.
///
/// This is the safe version of [`UnsafeCoroutineMut`], providing a mutable
/// awaiter reference to the closure.
pub trait CoroutineMut: UnsafeCoroutineMut + CoroutineMutSelfMut {
    /// Executes a closure with a mutable awaiter reference.
    fn exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for unsafely executing synchronous code with shared awaiter access.
///
/// # Safety
///
/// Implementations must ensure the awaiter is valid for the duration of the closure.
pub trait UnsafeCoroutine: UnsafeCoroutineMut + UnsafeCoroutineSelfMut {
    /// Executes a closure with a shared awaiter reference.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for safely executing synchronous code with shared awaiter access.
///
/// This is the primary coroutine trait, allowing synchronous closures to
/// block on futures within an async context.
///
/// # Example
///
/// ```ignore
/// use awaiter_trait::Coroutine;
///
/// async fn example<C: Coroutine>(coro: &C) {
///     let result = coro.exec(|awaiter| {
///         // Use awaiter to block on futures synchronously
///         42
///     }).await;
/// }
/// ```
pub trait Coroutine: UnsafeCoroutine + CoroutineMut + CoroutineSelfMut {
    /// Executes a closure with a shared awaiter reference.
    fn exec<T>(&self, f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T) -> impl Future<Output = T>;
}

/// A trait for unsafely executing with mutable self and mutable awaiter access.
pub trait UnsafeCoroutineMutSelfMut {
    /// Executes a closure with mutable access to both self and the awaiter.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for safely executing with mutable self and mutable awaiter access.
pub trait CoroutineMutSelfMut: UnsafeCoroutineMutSelfMut {
    /// Executes a closure with mutable access to both self and the awaiter.
    fn exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for unsafely executing with mutable self and shared awaiter access.
pub trait UnsafeCoroutineSelfMut: UnsafeCoroutineMutSelfMut {
    /// Executes a closure with mutable self but shared awaiter access.
    ///
    /// # Safety
    ///
    /// Callers must ensure the implementation's safety requirements are met.
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T>;
}

/// A trait for safely executing with mutable self and shared awaiter access.
pub trait CoroutineSelfMut: UnsafeCoroutineSelfMut + CoroutineMutSelfMut {
    /// Executes a closure with mutable self but shared awaiter access.
    fn exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T,
    ) -> impl Future<Output = T>;
}
impl<'a, T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMut for &'a T2 {
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<'a, T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMut for &'a mut T2 {
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<'a, T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMutSelfMut for &'a T2 {
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<'a, T2: UnsafeCoroutineMutSelfMut + ?Sized> UnsafeCoroutineMutSelfMut for &'a mut T2 {
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).unsafe_exec_mut_self_mut(f)
    }
}
impl<'a, T2: UnsafeCoroutine + ?Sized> UnsafeCoroutine for &'a T2 {
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<'a, T2: UnsafeCoroutine + ?Sized> UnsafeCoroutine for &'a mut T2 {
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<'a, T2: UnsafeCoroutine + ?Sized> UnsafeCoroutineSelfMut for &'a T2 {
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<'a, T2: UnsafeCoroutineSelfMut + ?Sized> UnsafeCoroutineSelfMut for &'a mut T2 {
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).unsafe_exec_self_mut(f)
    }
}

impl<'a, T2: CoroutineMut + ?Sized> CoroutineMut for &'a T2 {
    fn exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).exec_mut(f)
    }
}
impl<'a, T2: CoroutineMut + ?Sized> CoroutineMut for &'a mut T2 {
    fn exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).exec_mut(f)
    }
}
impl<'a, T2: CoroutineMut + ?Sized> CoroutineMutSelfMut for &'a T2 {
    fn exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).exec_mut(f)
    }
}
impl<'a, T2: CoroutineMutSelfMut + ?Sized> CoroutineMutSelfMut for &'a mut T2 {
    fn exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).exec_mut_self_mut(f)
    }
}
impl<'a, T2: Coroutine + ?Sized> Coroutine for &'a T2 {
    fn exec<T>(&self, f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T) -> impl Future<Output = T> {
        (&**self).exec(f)
    }
}
impl<'a, T2: Coroutine + ?Sized> Coroutine for &'a mut T2 {
    fn exec<T>(&self, f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T) -> impl Future<Output = T> {
        (&**self).exec(f)
    }
}
impl<'a, T2: Coroutine + ?Sized> CoroutineSelfMut for &'a T2 {
    fn exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).exec(f)
    }
}
impl<'a, T2: CoroutineSelfMut + ?Sized> CoroutineSelfMut for &'a mut T2 {
    fn exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).exec_self_mut(f)
    }
}

impl<T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMut for *const T2 {
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMut for *mut T2 {
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<T2: UnsafeCoroutineMut + ?Sized> UnsafeCoroutineMutSelfMut for *const T2 {
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec_mut(f)
    }
}
impl<T2: UnsafeCoroutineMutSelfMut + ?Sized> UnsafeCoroutineMutSelfMut for *mut T2 {
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).unsafe_exec_mut_self_mut(f)
    }
}
impl<T2: UnsafeCoroutine + ?Sized> UnsafeCoroutine for *const T2 {
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<T2: UnsafeCoroutine + ?Sized> UnsafeCoroutine for *mut T2 {
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<T2: UnsafeCoroutine + ?Sized> UnsafeCoroutineSelfMut for *const T2 {
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&**self).unsafe_exec(f)
    }
}
impl<T2: UnsafeCoroutineSelfMut + ?Sized> UnsafeCoroutineSelfMut for *mut T2 {
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        (&mut **self).unsafe_exec_self_mut(f)
    }
}
