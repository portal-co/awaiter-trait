use core::{mem::MaybeUninit, pin::pin};

use crate::*;
pub trait DynAwaiter: DynAwaiterMut + DynUnsafeAwaiter {
    fn dyn_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}

pub trait DynAwaiterMut: DynUnsafeAwaiterMut {
    fn dyn_await_mut(&mut self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}

pub trait DynUnsafeAwaiter: DynUnsafeAwaiterMut {
    unsafe fn dyn_unsafe_await(&self, f: Pin<&mut (dyn Future<Output = ()> + '_)>) -> ();
}
pub trait DynUnsafeAwaiterMut {
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