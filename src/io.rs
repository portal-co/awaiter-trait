use core::pin::{pin, Pin};

use embedded_io::ErrorType;

use crate::{AwaiterMut, CoroutineMutSelfMut};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Wrap<T, U> {
    pub wrapper: T,
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
