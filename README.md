# rt-endian-rw - Runtime Endian Read/Write
[![language](https://shields.io/badge/-Rust-D34516?style=flat&logo=rust)](https://rust-lang.org/)
![License](https://img.shields.io/badge/License-MIT%2D%2D0-blue)
[![crates.io](https://img.shields.io/crates/v/rt-endian-rw.svg)](https://crates.io/crates/rt-endian-rw)

A Rust library crate that extends the `Read` and `Write` traits to read and write Rust types with a runtime-specified endianness.

## Supported types
Signed and Unsigned 8-, 16-, 32-, 64-, and 128-bit integers, alongside f32 and f64.

## Usage

Import the extension traits to read and write integer and floating-point values.
Multi-byte values take an `Endianness` argument; single-byte values do not.

Writing and Reading a `u16` with `Endianness::Big` to a `std::io::Cursor`:
```rust
use std::io::Cursor;
use rt_endian_rw::{Endianness, ReadEndian, WriteEndian};

let mut cursor = Cursor::new(Vec::new());
cursor.write_u16(0x1234, Endianness::Big)?;

cursor.set_position(0);
assert_eq!(cursor.read_u16(Endianness::Big)?, 0x1234);
# Ok::<(), std::io::Error>(())
```

## Why use this?
Use this if you need to read or write types with runtime-specified endianness without any
wrapper or additional code. This crate has zero dependencies and is only a single file.

If you are looking to just read data with an Endianness specified at compile time, please use
[byteorder](https://crates.io/crates/byteorder).