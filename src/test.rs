use bytes::{Bytes, Buf};
use crate::TryBuf;

static TEST_SLICE: [u8;16] = u128::MAX.to_be_bytes();

#[test]
fn test_try_get_u8() {
    let mut bytes = Bytes::from_static(&[]);
    let a = bytes.try_get_u8();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 0);

    let mut bytes = Bytes::from(&TEST_SLICE[..1]);
    let a = bytes.try_get_u8();
    if a.is_err() {
        panic!();
    }
    assert_eq!(a.unwrap(), u8::MAX);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_i8() {
    let mut bytes = Bytes::from_static(&[]);
    let a = bytes.try_get_i8();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 0);

    let mut bytes = Bytes::from(&TEST_SLICE[..1]);
    let a = bytes.try_get_i8();
    if a.is_err() {
        panic!();
    }

    assert_eq!(a.unwrap(), -0x01);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_u16() {
    let mut bytes = Bytes::from(&TEST_SLICE[..1]);
    let a = bytes.try_get_u16();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 1);

    let mut bytes = Bytes::from(&TEST_SLICE[..2]);
    let a = bytes.try_get_u16();
    if a.is_err() {
        panic!();
    }
    assert_eq!(a.unwrap(), u16::MAX);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_i16() {
    let mut bytes = Bytes::from(&TEST_SLICE[..1]);
    let a = bytes.try_get_i16();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 1);

    let mut bytes = Bytes::from(&TEST_SLICE[..2]);
    let a = bytes.try_get_i16();
    if a.is_err() {
        panic!();
    }

    assert_eq!(a.unwrap(), -1);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_u32() {
    let mut bytes = Bytes::from(&TEST_SLICE[..3]);
    let a = bytes.try_get_u32();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 3);

    let mut bytes = Bytes::from(&TEST_SLICE[..4]);
    let a = bytes.try_get_u32();
    if a.is_err() {
        panic!();
    }
    assert_eq!(a.unwrap(), u32::MAX);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_i32() {
    let mut bytes = Bytes::from(&TEST_SLICE[..3]);
    let a = bytes.try_get_i32();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 3);

    let mut bytes = Bytes::from(&TEST_SLICE[..4]);
    let a = bytes.try_get_i32();
    if a.is_err() {
        panic!();
    }

    assert_eq!(a.unwrap(), -1);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_u64() {
    let mut bytes = Bytes::from(&TEST_SLICE[..7]);
    let a = bytes.try_get_u64();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 7);

    let mut bytes = Bytes::from(&TEST_SLICE[..8]);
    let a = bytes.try_get_u64();
    if a.is_err() {
        panic!();
    }
    assert_eq!(a.unwrap(), u64::MAX);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_i64() {
    let mut bytes = Bytes::from(&TEST_SLICE[..7]);
    let a = bytes.try_get_i64();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 7);

    let mut bytes = Bytes::from(&TEST_SLICE[..8]);
    let a = bytes.try_get_i64();
    if a.is_err() {
        panic!();
    }

    assert_eq!(a.unwrap(), -1);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_u128() {
    let mut bytes = Bytes::from(&TEST_SLICE[..15]);
    let a = bytes.try_get_u128();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 15);

    let mut bytes = Bytes::from(TEST_SLICE.as_slice());
    let a = bytes.try_get_u128();
    if a.is_err() {
        panic!();
    }
    assert_eq!(a.unwrap(), u128::MAX);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_try_get_i128() {
    let mut bytes = Bytes::from(&TEST_SLICE[..15]);
    let a = bytes.try_get_i128();
    if a.is_ok() {
        panic!();
    }
    assert_eq!(bytes.remaining(), 15);

    let mut bytes = Bytes::from(TEST_SLICE.as_slice());
    let a = bytes.try_get_i128();
    if a.is_err() {
        panic!();
    }

    assert_eq!(a.unwrap(), -1);
    assert_eq!(bytes.remaining(), 0);
}
