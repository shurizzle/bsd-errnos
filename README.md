# bsd-errnos

[![Crates.io](https://img.shields.io/crates/v/bsd-errnos?style=for-the-badge)](https://crates.io/crates/bsd-errnos)
[![docs.rs](https://img.shields.io/docsrs/bsd-errnos?style=for-the-badge)](https://docs.rs/bsd-errnos)
![Crates.io](https://img.shields.io/crates/l/bsd-errnos?style=for-the-badge)

Cross-arch enumeration of BSDs error numbers that may vary across archs.

### Feature flags

- `std`: enable std support (dealing with std::io::Error).
- `iter`: enable `Errno::iter()` function.

### #![no_std]

Enable `#![no_std]` support by disabling the default `std` feature:

```toml
[dependencies]
bsd-errnos = { version = "*", default-features = false }
```

# Code generation

All the code is generated by the an inner crate in the `errno-gen` directory
except for the `src/lib.rs` file, so don't touch auto generated files please.

### MSRV

1.32.0
