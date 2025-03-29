#![no_std]

use core::pin::Pin;
pub trait Awaiter: AwaiterMut + UnsafeAwaiter {
    fn r#await<T>(&self, f: Pin<&mut dyn Future<Output = T>>) -> T;
}
pub trait AwaiterMut: UnsafeAwaiterMut {
    fn await_mut<T>(&mut self, f: Pin<&mut dyn Future<Output = T>>) -> T;
}
pub trait UnsafeAwaiter: UnsafeAwaiterMut {
    unsafe fn unsafe_await<T>(&self, f: Pin<&mut dyn Future<Output = T>>) -> T;
}
pub trait UnsafeAwaiterMut {
    unsafe fn unsafe_await_mut<T>(&mut self, f: Pin<&mut dyn Future<Output = T>>) -> T;
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
                fn await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut dyn $crate::__::core::future::Future<Output = T>>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            impl<$($g),*> $crate::UnsafeAwaiter for $t{
                unsafe fn unsafe_await<T>(&self, f: $crate::__::core::pin::Pin<&mut dyn $crate::__::core::future::Future<Output = T>>) -> T{
                    <Self as $crate::Awaiter>::r#await(self,f)
                }
            }
            $crate::autoimpl!(<$($g),*> $t as AwaiterMut);

        };
    };
    (<$($g:ident),*> $t:ty as UnsafeAwaiter) => {
        const _: () = {
            impl<$($g),*> $crate::UnsafeAwaiterMut for $t{
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut dyn $crate::__::core::future::Future<Output = T>>) -> T{
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
                unsafe fn unsafe_await_mut<T>(&mut self, f: $crate::__::core::pin::Pin<&mut dyn $crate::__::core::future::Future<Output = T>>) -> T{
                   <Self as $crate::AwaiterMut>::await_mut(self,f)
                }
            }
        };
    };
}
