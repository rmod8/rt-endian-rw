#![doc = include_str!("../README.md")]

/// Represents how multi-byte numbers should be ordered. Most computers use [`Endianness::Little`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// 0x1234 is read as 0x34 and 0x12, a [`u16`] value of `13330`.
    Little,
    /// 0x1234 is read as 0x12 and 0x34, a [`u16`] value of `4660`.
    Big,
}

impl std::ops::Not for Endianness {
    type Output = Endianness;

    fn not(self) -> Self::Output {
        match self {
            Endianness::Little => Endianness::Big,
            Endianness::Big => Endianness::Little,
        }
    }
}

/// Reads the specified type from the [`std::io::Read`] implementor and returns
/// it in a [`std::io::Result`].
macro_rules! read_type {
    ($trait_name:ident, $type_name:ident, $func_name:ident) => {
        #[doc = concat!("Reads a [`", stringify!($type_name), "`] with the specified [`Endianness`].")]
        #[inline]
        fn $func_name(&mut self, endianness: $crate::Endianness) -> std::io::Result<$type_name> {
            let mut buf = [0u8; size_of::<$type_name>()];
            self.read_exact(&mut buf)?;
            match endianness {
                $crate::Endianness::Little => Ok($type_name::from_le_bytes(buf)),
                $crate::Endianness::Big => Ok($type_name::from_be_bytes(buf)),
            }
        }
    };
}

/// Reads the specified 1-byte type ([`u8`] or [`i8`]) from the [`std::io::Read`] implementor and returns
/// it in a [`std::io::Result`].
///
/// Does not take an Endianness parameter as bytes are not endian-sensitive.
macro_rules! read_byte {
    ($trait_name:ident, $type_name:ident, $func_name:ident) => {
        #[doc = concat!("Reads a [`", stringify!($type_name), "`].")]
        #[inline]
        fn $func_name(&mut self) -> std::io::Result<$type_name> {
            let mut buf = [0u8; 1];
            self.read_exact(&mut buf)?;
            Ok($type_name::from_le_bytes(buf))
        }
    };
}

/// Writes the specified type to the [`std::io::Write`] implementor and returns
/// [`()`](https://doc.rust-lang.org/std/primitive.unit.html) in a [`std::io::Result`].
macro_rules! write_type {
    ($type_name:ident, $func_name:ident) => {
        #[doc = concat!("Writes the specified [`", stringify!($type_name), "`] with the specified [`Endianness`].")]
        #[inline]
        fn $func_name(
            &mut self,
            value: $type_name,
            endianness: $crate::Endianness,
        ) -> std::io::Result<()> {
            let bytes = match endianness {
                $crate::Endianness::Little => value.to_le_bytes(),
                $crate::Endianness::Big => value.to_be_bytes(),
            };
            self.write_all(&bytes)
        }
    };
}

/// Writes the specified 1-byte type ([`u8`] or [`i8`]) to the [`std::io::Write`] implementor and returns
/// [`()`](https://doc.rust-lang.org/std/primitive.unit.html) in a [`std::io::Result`].
macro_rules! write_byte {
    ($type_name:ident, $func_name:ident) => {
        #[doc = concat!("Writes the specified [`", stringify!($type_name), "`].")]
        #[inline]
        fn $func_name(&mut self, value: $type_name) -> std::io::Result<()> {
            let bytes = value.to_le_bytes();
            self.write_all(&bytes)
        }
    };
}

/// Extends any struct implementing [`std::io::Read`] to allow the reading of specific types with a specified [`Endianness`].
pub trait ReadEndian: std::io::Read {
    read_byte!(ReadEndian, u8, read_u8);
    read_type!(ReadEndian, u16, read_u16);
    read_type!(ReadEndian, u32, read_u32);
    read_type!(ReadEndian, u64, read_u64);
    read_type!(ReadEndian, u128, read_u128);

    read_byte!(ReadEndian, i8, read_i8);
    read_type!(ReadEndian, i16, read_i16);
    read_type!(ReadEndian, i32, read_i32);
    read_type!(ReadEndian, i64, read_i64);
    read_type!(ReadEndian, i128, read_i128);

    read_type!(ReadEndian, f32, read_f32);
    read_type!(ReadEndian, f64, read_f64);
}

/// Extends any struct implementing [`std::io::Write`] to allow the writing of specific types with a specified [`Endianness`].
pub trait WriteEndian: std::io::Write {
    write_byte!(u8, write_u8);
    write_type!(u16, write_u16);
    write_type!(u32, write_u32);
    write_type!(u64, write_u64);
    write_type!(u128, write_u128);

    write_byte!(i8, write_i8);
    write_type!(i16, write_i16);
    write_type!(i32, write_i32);
    write_type!(i64, write_i64);
    write_type!(i128, write_i128);

    write_type!(f32, write_f32);
    write_type!(f64, write_f64);
}

impl<R: std::io::Read + ?Sized> ReadEndian for R {}
impl<W: std::io::Write + ?Sized> WriteEndian for W {}

#[cfg(test)]
#[allow(missing_docs)]
mod tests {

    const U8: u8 = 0xFFu8;
    const U8_BYTES: [u8; 1] = [U8];

    const U16: u16 = 0x00FFu16;
    const U16_LE_BYTES: [u8; 2] = U16.to_le_bytes();
    const U16_BE_BYTES: [u8; 2] = U16.to_be_bytes();

    const U32: u32 = 0x0000FFFFu32;
    const U32_LE_BYTES: [u8; 4] = U32.to_le_bytes();
    const U32_BE_BYTES: [u8; 4] = U32.to_be_bytes();

    const U64: u64 = 0x00000000FFFFFFFFu64;
    const U64_LE_BYTES: [u8; 8] = U64.to_le_bytes();
    const U64_BE_BYTES: [u8; 8] = U64.to_be_bytes();

    const U128: u128 = 0x0000000000000000FFFFFFFFFFFFFFFFu128;
    const U128_LE_BYTES: [u8; 16] = U128.to_le_bytes();
    const U128_BE_BYTES: [u8; 16] = U128.to_be_bytes();

    const F32: f32 = 1f32;
    const F32_LE_BYTES: [u8; 4] = F32.to_le_bytes();
    const F32_BE_BYTES: [u8; 4] = F32.to_be_bytes();

    const F64: f64 = 1f64;
    const F64_LE_BYTES: [u8; 8] = F64.to_le_bytes();
    const F64_BE_BYTES: [u8; 8] = F64.to_be_bytes();

    #[cfg(test)]
    mod read {
        use super::super::*;
        use super::*;
        use std::io::Cursor;

        #[test]
        fn read_u8() {
            let mut cursor = Cursor::new(U8_BYTES);
            let value = cursor.read_u8().unwrap();
            assert_eq!(value, U8 as u8);
        }

        #[test]
        fn read_u16_le() {
            let mut cursor = Cursor::new(U16_LE_BYTES);
            let value = cursor.read_u16(Endianness::Little).unwrap();
            assert_eq!(value, U16);
        }

        #[test]
        fn read_u16_be() {
            let mut cursor = Cursor::new(U16_BE_BYTES);
            let value = cursor.read_u16(Endianness::Big).unwrap();
            assert_eq!(value, U16);
        }

        #[test]
        fn read_u32_le() {
            let mut cursor = Cursor::new(U32_LE_BYTES);
            let value = cursor.read_u32(Endianness::Little).unwrap();
            assert_eq!(value, U32);
        }

        #[test]
        fn read_u32_be() {
            let mut cursor = Cursor::new(U32_BE_BYTES);
            let value = cursor.read_u32(Endianness::Big).unwrap();
            assert_eq!(value, U32);
        }

        #[test]
        fn read_u64_le() {
            let mut cursor = Cursor::new(U64_LE_BYTES);
            let value = cursor.read_u64(Endianness::Little).unwrap();
            assert_eq!(value, U64);
        }

        #[test]
        fn read_u64_be() {
            let mut cursor = Cursor::new(U64_BE_BYTES);
            let value = cursor.read_u64(Endianness::Big).unwrap();
            assert_eq!(value, U64);
        }

        #[test]
        fn read_u128_le() {
            let mut cursor = Cursor::new(U128_LE_BYTES);
            let value = cursor.read_u128(Endianness::Little).unwrap();
            assert_eq!(value, U128);
        }

        #[test]
        fn read_u128_be() {
            let mut cursor = Cursor::new(U128_BE_BYTES);
            let value = cursor.read_u128(Endianness::Big).unwrap();
            assert_eq!(value, U128);
        }

        #[test]
        fn read_i8() {
            let mut cursor = Cursor::new(U8_BYTES);
            let value = cursor.read_i8().unwrap();
            assert_eq!(value, U8 as i8);
        }

        #[test]
        fn read_i16_le() {
            let mut cursor = Cursor::new(U16_LE_BYTES);
            let value = cursor.read_i16(Endianness::Little).unwrap();
            assert_eq!(value, U16 as i16);
        }

        #[test]
        fn read_i16_be() {
            let mut cursor = Cursor::new(U16_BE_BYTES);
            let value = cursor.read_i16(Endianness::Big).unwrap();
            assert_eq!(value, U16 as i16);
        }

        #[test]
        fn read_i32_le() {
            let mut cursor = Cursor::new(U32_LE_BYTES);
            let value = cursor.read_i32(Endianness::Little).unwrap();
            assert_eq!(value, U32 as i32);
        }

        #[test]
        fn read_i32_be() {
            let mut cursor = Cursor::new(U32_BE_BYTES);
            let value = cursor.read_i32(Endianness::Big).unwrap();
            assert_eq!(value, U32 as i32);
        }

        #[test]
        fn read_i64_le() {
            let mut cursor = Cursor::new(U64_LE_BYTES);
            let value = cursor.read_i64(Endianness::Little).unwrap();
            assert_eq!(value, U64 as i64);
        }

        #[test]
        fn read_i64_be() {
            let mut cursor = Cursor::new(U64_BE_BYTES);
            let value = cursor.read_i64(Endianness::Big).unwrap();
            assert_eq!(value, U64 as i64);
        }

        #[test]
        fn read_i128_le() {
            let mut cursor = Cursor::new(U128_LE_BYTES);
            let value = cursor.read_i128(Endianness::Little).unwrap();
            assert_eq!(value, U128 as i128);
        }

        #[test]
        fn read_i128_be() {
            let mut cursor = Cursor::new(U128_BE_BYTES);
            let value = cursor.read_i128(Endianness::Big).unwrap();
            assert_eq!(value, U128 as i128);
        }

        #[test]
        fn read_f32_le() {
            let mut cursor = Cursor::new(F32_LE_BYTES);
            let value = cursor.read_f32(Endianness::Little).unwrap();
            assert_eq!(value, F32);
        }

        #[test]
        fn read_f32_be() {
            let mut cursor = Cursor::new(F32_BE_BYTES);
            let value = cursor.read_f32(Endianness::Big).unwrap();
            assert_eq!(value, F32);
        }

        #[test]
        fn read_f64_le() {
            let mut cursor = Cursor::new(F64_LE_BYTES);
            let value = cursor.read_f64(Endianness::Little).unwrap();
            assert_eq!(value, F64);
        }

        #[test]
        fn read_f64_be() {
            let mut cursor = Cursor::new(F64_BE_BYTES);
            let value = cursor.read_f64(Endianness::Big).unwrap();
            assert_eq!(value, F64);
        }
    }

    #[cfg(test)]
    mod write {
        use super::super::*;
        use super::*;
        use std::io::Cursor;

        #[test]
        fn write_u8() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u8(U8).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u8().unwrap(), U8);
        }

        #[test]
        fn write_u16_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u16(U16, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u16(Endianness::Little).unwrap(), U16);
        }

        #[test]
        fn write_u16_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u16(U16, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u16(Endianness::Big).unwrap(), U16);
        }

        #[test]
        fn write_u32_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u32(U32, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u32(Endianness::Little).unwrap(), U32);
        }

        #[test]
        fn write_u32_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u32(U32, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u32(Endianness::Big).unwrap(), U32);
        }

        #[test]
        fn write_u64_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u64(U64, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u64(Endianness::Little).unwrap(), U64);
        }

        #[test]
        fn write_u64_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u64(U64, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u64(Endianness::Big).unwrap(), U64);
        }

        #[test]
        fn write_u128_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u128(U128, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u128(Endianness::Little).unwrap(), U128);
        }

        #[test]
        fn write_u128_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_u128(U128, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_u128(Endianness::Big).unwrap(), U128);
        }

        #[test]
        fn write_i8() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i8(U8 as i8).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i8().unwrap(), U8 as i8);
        }

        #[test]
        fn write_i16_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i16(U16 as i16, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i16(Endianness::Little).unwrap(), U16 as i16);
        }

        #[test]
        fn write_i16_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i16(U16 as i16, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i16(Endianness::Big).unwrap(), U16 as i16);
        }

        #[test]
        fn write_i32_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i32(U32 as i32, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i32(Endianness::Little).unwrap(), U32 as i32);
        }

        #[test]
        fn write_i32_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i32(U32 as i32, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i32(Endianness::Big).unwrap(), U32 as i32);
        }

        #[test]
        fn write_i64_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i64(U64 as i64, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i64(Endianness::Little).unwrap(), U64 as i64);
        }

        #[test]
        fn write_i64_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i64(U64 as i64, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i64(Endianness::Big).unwrap(), U64 as i64);
        }

        #[test]
        fn write_i128_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i128(U128 as i128, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i128(Endianness::Little).unwrap(), U128 as i128);
        }

        #[test]
        fn write_i128_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_i128(U128 as i128, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_i128(Endianness::Big).unwrap(), U128 as i128);
        }

        #[test]
        fn write_f32_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_f32(F32, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_f32(Endianness::Little).unwrap(), F32);
        }

        #[test]
        fn write_f32_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_f32(F32, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_f32(Endianness::Big).unwrap(), F32);
        }

        #[test]
        fn write_f64_le() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_f64(F64, Endianness::Little).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_f64(Endianness::Little).unwrap(), F64);
        }

        #[test]
        fn write_f64_be() {
            let mut cursor = Cursor::new(Vec::new());
            cursor.write_f64(F64, Endianness::Big).unwrap();
            cursor.set_position(0);
            assert_eq!(cursor.read_f64(Endianness::Big).unwrap(), F64);
        }
    }
}
