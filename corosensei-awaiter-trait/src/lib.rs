//! # corosensei-awaiter-trait
//!
//! This crate provides an implementation of `awaiter-trait`'s [`awaiter_trait::Coroutine`]
//! trait using the [`corosensei`] stackful coroutine library.
//!
//! ## Overview
//!
//! The [`Stacc`] type implements the `Coroutine` trait, allowing you to execute
//! synchronous code that can block on futures using stackful coroutines.
//! This is useful for scenarios where you need to call async code from
//! synchronous contexts without blocking the entire thread.
//!
//! ## Usage
//!
//! ```ignore
//! use corosensei_awaiter_trait::Stacc;
//! use awaiter_trait::Coroutine;
//!
//! async fn example() {
//!     let stack_provider = || corosensei::stack::DefaultStack::new(64 * 1024).unwrap();
//!     let stacc = Stacc { via: &stack_provider };
//!
//!     let result = stacc.exec(|awaiter| {
//!         // Synchronous code that can use the awaiter to block on futures
//!         42
//!     }).await;
//! }
//! ```
//!
//! ## Features
//!
//! - `no_std` compatible (requires `alloc` for stack allocation)
//! - Uses `corosensei` for efficient stackful coroutines
//! - Implements the full `Coroutine` trait hierarchy

#![no_std]

use core::{
    future::Future,
    mem::{MaybeUninit, replace, transmute},
    pin::Pin,
    task::{Context, Poll, Waker},
};

use awaiter_trait::r#dyn::DynAwaiter;
use corosensei::{Coroutine, Yielder};

use spin::Mutex;

/// Internal future that wraps a corosensei coroutine.
struct CoroImpl<T, Stack: corosensei::stack::Stack + Unpin> {
    cor: Coroutine<Waker, (), T, Stack>,
}

impl<T, Stack: corosensei::stack::Stack + Unpin> Future for CoroImpl<T, Stack> {
    type Output = T;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let s = self.get_mut();
        match s.cor.resume(cx.waker().clone()) {
            corosensei::CoroutineResult::Yield(_) => Poll::Pending,
            corosensei::CoroutineResult::Return(a) => Poll::Ready(a),
        }
    }
}

/// Internal awaiter implementation that uses a corosensei yielder.
struct Awaiter<'a> {
    y: &'a Yielder<Waker, ()>,
    w: spin::Mutex<Waker>,
}

impl awaiter_trait::Awaiter for Awaiter<'_> {
    fn r#await<T>(&self, mut f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        loop {
            let s = self.y.suspend(());
            let waker = {
                let mut lock = self.w.lock();
                replace(&mut *lock, s)
            };
            match f.as_mut().poll(&mut Context::from_waker(&waker)) {
                Poll::Ready(r) => return r,
                Poll::Pending => {}
            };
        }
    }
}

awaiter_trait::autoimpl!(<>Awaiter<'_> as Awaiter);

/// A coroutine provider that creates stackful coroutines for awaiting futures.
///
/// This type implements [`awaiter_trait::Coroutine`] using corosensei's stackful
/// coroutines. Each call to `exec` creates a new coroutine with a stack
/// provided by the `via` closure.
///
/// # Type Parameters
///
/// - `Stack`: The stack type to use for coroutines, must implement
///   `corosensei::stack::Stack + Unpin`
///
/// # Example
///
/// ```ignore
/// use corosensei_awaiter_trait::Stacc;
/// use awaiter_trait::Coroutine;
///
/// async fn example() {
///     let stack_provider = || corosensei::stack::DefaultStack::new(64 * 1024).unwrap();
///     let stacc = Stacc { via: &stack_provider };
///
///     stacc.exec(|awaiter| {
///         // Use awaiter here
///     }).await;
/// }
/// ```
pub struct Stacc<'a, Stack: corosensei::stack::Stack + Unpin> {
    /// A closure that provides a new stack for each coroutine.
    pub via: &'a (dyn Fn() -> Stack + 'a),
}

/// Helper trait alias for `corosensei::stack::Stack + Unpin`.
pub trait UPS: corosensei::stack::Stack + Unpin {}
impl<T: corosensei::stack::Stack + Unpin + ?Sized> UPS for T {}
awaiter_trait::autoimpl!(<Stack: UPS>Stacc<'_,Stack> as Coroutine);
impl<Stack: corosensei::stack::Stack + Unpin> awaiter_trait::Coroutine for Stacc<'_, Stack> {
    fn exec<T>(
        &self,
        f: impl FnOnce(&(dyn awaiter_trait::r#dyn::DynAwaiter + '_)) -> T,
    ) -> impl Future<Output = T> {
        async move {
            let mut t = MaybeUninit::uninit();
            let mut f = match &mut t {
                mut t => match Some(f) {
                    mut a => move |b: &(dyn DynAwaiter + '_)| {
                        t.write(unsafe { a.take().unwrap_unchecked() }(b));
                    },
                },
            };
            let f: &mut (dyn FnMut(&(dyn awaiter_trait::r#dyn::DynAwaiter + '_)) -> () + '_) =
                &mut f;
            let mut f: &mut (dyn FnMut(&(dyn awaiter_trait::r#dyn::DynAwaiter + '_)) -> ()) =
                unsafe { transmute(f) };
            CoroImpl {
                cor: Coroutine::with_stack((self.via)(), move |a, _| {
                    f(&Awaiter {
                        y: a,
                        w: Mutex::new(Waker::noop().clone()),
                    })
                }),
            }
            .await;
            unsafe { t.assume_init() }
        }
    }
}
