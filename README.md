This crate provides no-panic API for any type `T` impl `bytes::Buf`,
which is mentioned in [issue#254](https://github.com/tokio-rs/bytes/issues/254).

issues and PRs are welcome.

# example

```rust
use bytes::{Bytes, Buf};
use crate::TryBuf;

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
```
