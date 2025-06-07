#![no_std]

use core::pin::Pin;
pub mod r#dyn;
use r#dyn::*;
#[cfg(feature = "embedded-io")]
pub mod io;
pub trait Awaiter: AwaiterMut + UnsafeAwaiter {
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}

pub trait AwaiterMut: UnsafeAwaiterMut {
    fn await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}

pub trait UnsafeAwaiter: UnsafeAwaiterMut {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}
pub trait UnsafeAwaiterMut {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T;
}
#[doc(hidden)]
pub mod __ {
    pub use core;
}
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
pub trait UnsafeCoroutineMut: UnsafeCoroutineMutSelfMut {
    unsafe fn unsafe_exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait CoroutineMut: UnsafeCoroutineMut + CoroutineMutSelfMut {
    fn exec_mut<T>(
        &self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait UnsafeCoroutine: UnsafeCoroutineMut + UnsafeCoroutineSelfMut {
    unsafe fn unsafe_exec<T>(
        &self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait Coroutine: UnsafeCoroutine + CoroutineMut + CoroutineSelfMut {
    fn exec<T>(&self, f: impl FnOnce(&(dyn DynAwaiter + '_)) -> T) -> impl Future<Output = T>;
}
pub trait UnsafeCoroutineMutSelfMut {
    unsafe fn unsafe_exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynUnsafeAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait CoroutineMutSelfMut: UnsafeCoroutineMutSelfMut {
    fn exec_mut_self_mut<T>(
        &mut self,
        f: impl FnOnce(&mut (dyn DynAwaiterMut + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait UnsafeCoroutineSelfMut: UnsafeCoroutineMutSelfMut {
    unsafe fn unsafe_exec_self_mut<T>(
        &mut self,
        f: impl FnOnce(&(dyn DynUnsafeAwaiter + '_)) -> T,
    ) -> impl Future<Output = T>;
}
pub trait CoroutineSelfMut: UnsafeCoroutineSelfMut + CoroutineMutSelfMut {
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
