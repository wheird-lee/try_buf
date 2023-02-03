//! This crate provides no-panic API for any type `T` impl `bytes::Buf`,
//! which is mentioned in [issue#254](https://github.com/tokio-rs/bytes/issues/254).
//!
//! # example
//!
//! ```
//! use bytes::{Bytes, Buf};
//! use crate::TryBuf;
//!
//! let mut bytes = Bytes::from_static(&[1, 2, 3, 4]);
//!         
//! let a = bytes.try_get_u16().unwrap();
//! assert_eq!(a, 0x0102);
//!
//! let b = bytes.try_get_u32()
//!     .unwrap_or_else(|e| {
//!         println!("fail to get u32: {}", e);
//!         0
//!     });
//! assert_eq!(b, 0);
//! assert_eq!(2, bytes.remaining());
//! ```
//!

use bytes::{Buf, Bytes};
pub use paste::paste;

#[macro_use]
mod macros;

pub type Result<T> = std::result::Result<T, ErrorKind>;

// #[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    EOF,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "Unexpected EOF"),
        }
    }
}

impl std::error::Error for ErrorKind {}

pub trait TryBuf {
    define_try_get!(u16 u32 u64 u128 i16 i32 i64 i128 f32 f64);
    define_try_get!(@def_be u8 i8);

    fn try_copy_to_bytes(&mut self, len: usize) -> Result<Bytes>;

    fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<()>;
}

impl<B: Buf> TryBuf for B {
    impl_try_get!(u16 u32 u64 u128 i16 i32 i64 i128 f32 f64);
    impl_try_get!(@impls_be u8 i8);

    fn try_copy_to_bytes(&mut self, len: usize) -> Result<Bytes> {
        if len <= self.remaining() {
            Ok(self.copy_to_bytes(len))
        } else {
            Err(ErrorKind::EOF)
        }
    }

    fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<()> {
        if dst.len() <= self.remaining() {
            Ok(self.copy_to_slice(dst))
        } else {
            Err(ErrorKind::EOF)
        }
    }
}

#[cfg(test)]
mod test {

    use bytes::{Bytes, Buf};
    use crate::TryBuf;

    #[test]
    fn test_try_get() {
        let mut bytes = Bytes::from_static(&[1, 2, 3, 4]);

        let a = bytes.try_get_u16().unwrap();
        assert_eq!(a, 0x0102);

        let b = bytes.try_get_u32()
            .unwrap_or_else(|e| {
                println!("fail to get u32: {}", e);
                0
            });
        assert_eq!(b, 0);
        assert_eq!(2, bytes.remaining());

        let c = bytes.try_get_u16().unwrap();
        assert_eq!(c, 0x0304);
    }
}
