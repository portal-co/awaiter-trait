//! Integration with `embedded-io` and `embedded-io-async` crates.
//!
//! This module provides a [`Wrap`] type that bridges async I/O types with
//! synchronous I/O interfaces. It allows you to use async readers/writers
//! with synchronous code by providing an awaiter to block on async operations.
//!
//! # Example
//!
//! ```ignore
//! use awaiter_trait::io::Wrap;
//! use embedded_io::Read;
//!
//! fn read_sync<A: awaiter_trait::AwaiterMut, R: embedded_io_async::Read>(
//!     awaiter: A,
//!     reader: R,
//! ) -> Result<Vec<u8>, R::Error> {
//!     let mut wrapped = Wrap { wrapper: awaiter, wrapped: reader };
//!     let mut buf = [0u8; 256];
//!     let n = wrapped.read(&mut buf)?;
//!     Ok(buf[..n].to_vec())
//! }
//! ```

use core::pin::{pin, Pin};

use embedded_io::ErrorType;

use crate::{AwaiterMut, CoroutineMutSelfMut};

/// A wrapper that combines an awaiter with an async I/O type.
///
/// This struct allows using async I/O types (`embedded_io_async::Read`,
/// `embedded_io_async::Write`) through synchronous interfaces
/// (`embedded_io::Read`, `embedded_io::Write`) by using the awaiter
/// to block on async operations.
///
/// # Type Parameters
///
/// - `T`: The awaiter type, must implement [`AwaiterMut`]
/// - `U`: The wrapped async I/O type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Wrap<T, U> {
    /// The awaiter used to block on async operations.
    pub wrapper: T,
    /// The wrapped async I/O type.
    pub wrapped: U,
}
impl<T,U: ErrorType> ErrorType for Wrap<T,U>{
    type Error = U::Error;
}
impl<T: AwaiterMut,U: embedded_io_async::Read> embedded_io::Read for Wrap<T,U>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let Self { wrapper, wrapped } = self;
        let mut x = async move{
            wrapped.read(buf).await
        };
        let x: Pin<&mut (dyn Future<Output = Result<usize,U::Error>> + '_)> = unsafe{Pin::new_unchecked(&mut x)};
        wrapper.await_mut(x)
    }
}
impl<T: AwaiterMut,U: embedded_io_async::Write> embedded_io::Write for Wrap<T,U>{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let Self { wrapper, wrapped } = self;
        let mut x = async move{
            wrapped.write(buf).await
        };
        let x: Pin<&mut (dyn Future<Output = Result<usize,U::Error>> + '_)> = unsafe{Pin::new_unchecked(&mut x)};
        wrapper.await_mut(x)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let Self { wrapper, wrapped } = self;
        let mut x = async move{
            wrapped.flush().await
        };
        let x: Pin<&mut (dyn Future<Output = Result<(),U::Error>> + '_)> = unsafe{Pin::new_unchecked(&mut x)};
        wrapper.await_mut(x)
    }
}
