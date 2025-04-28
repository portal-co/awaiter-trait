#![no_std]

use core::pin::Pin;
pub mod r#dyn;
pub trait Awaiter: AwaiterMut + UnsafeAwaiter {
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T>  + '_)>) -> T;
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
    (<$($g:ident),*> $t:ty as Awaiter) => {
        const _: () = {
            impl<$($g),*> $crate::AwaiterMut for $t{
                fn await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            impl<$($g),*> $crate::UnsafeAwaiter for $t{
                unsafe fn unsafe_await<T>(&self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            $crate::autoimpl!(<$($g),*> $t as AwaiterMut);

        };
    };
    (<$($g:ident),*> $t:ty as UnsafeAwaiter) => {
        const _: () = {
            impl<$($g),*> $crate::UnsafeAwaiterMut for $t{
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                    unsafe{
                        <Self as $crate::UnsafeAwaiter>::unsafe_await(self,f)
                    }
                }
            }
        };
    };
    (<$($g:ident),*> $t:ty as AwaiterMut) => {
        const _: () = {
            impl<$($g),*> $crate::UnsafeAwaiterMut for $t{
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut (dyn $crate::__::core::future::Future<Output = T> + '_)>) -> T{
                   <Self as $crate::AwaiterMut>::await_mut(self,f)
                }
            }
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
