use crate::{AtReadable, AtWritable, HayesError, impls::BufferWriter};
use core::fmt::Write;

macro_rules! impl_uint {
    ($($ty:ty),*) => {
        $(
            impl<'at> AtReadable<'at> for $ty {
                fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError> {
                    if input.is_empty() {
                        return Err(HayesError::UnexpectedEnd);
                    }

                    let mut value: $ty = 0;
                    let mut consumed = 0;

                    for &byte in input.iter() {
                        if !byte.is_ascii_digit() {
                            break;
                        }

                        #[allow(clippy::cast_lossless)]
                        let digit = (byte - b'0') as $ty;
                        value = value
                            .checked_mul(10)
                            .and_then(|v| v.checked_add(digit))
                            .ok_or(HayesError::ParseError)?;
                        consumed += 1;
                    }

                    if consumed == 0 {
                        return Err(HayesError::ParseError);
                    }

                    Ok((value, consumed))
                }
            }

            impl AtWritable for $ty {
                fn write(&self, output: &mut [u8]) -> Result<usize, HayesError> {
                    let available = output.len();

                    let mut writer = BufferWriter::new(output);
                    write!(&mut writer, "{}", self).map_err(|_| HayesError::InsufficientBuffer {
                        required: if *self == 0 {
                            1
                        } else {
                            self.ilog10() as usize + 1
                        },
                        available,
                    })?;

                    Ok(writer.written())
                }
            }
        )*
    };
}

impl_uint!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        assert_eq!(u8::read(b"0"), Ok((0, 1)));
        assert_eq!(u8::read(b"123"), Ok((123, 3)));
        assert_eq!(u8::read(b"255"), Ok((255, 3)));
        assert_eq!(u8::read(b"42,"), Ok((42, 2)));
    }

    #[test]
    fn test_write_u8() {
        let mut buf = [0u8; 10];
        assert_eq!(0u8.write(&mut buf), Ok(1));
        assert_eq!(&buf[..1], b"0");

        assert_eq!(123u8.write(&mut buf), Ok(3));
        assert_eq!(&buf[..3], b"123");
    }

    #[test]
    fn test_write_large_uint() {
        let mut buf = [0u8; 50];
        assert_eq!(u64::MAX.write(&mut buf), Ok(20));
        assert_eq!(&buf[..20], b"18446744073709551615");

        assert_eq!(u128::MAX.write(&mut buf), Ok(39));
        assert_eq!(&buf[..39], b"340282366920938463463374607431768211455");
    }

    #[test]
    fn test_insufficient_buffer() {
        let mut buf = [0u8; 2];

        // 42 needs 2 bytes, buffer has 2 - should succeed
        assert_eq!(42u8.write(&mut buf), Ok(2));

        // 255 needs 3 bytes, buffer has 2 - should fail with required=3
        assert_eq!(
            255u8.write(&mut buf),
            Err(HayesError::InsufficientBuffer {
                required: 3,
                available: 2,
            })
        );

        // 0 needs 1 byte
        let mut buf0 = [0u8; 0];
        assert_eq!(
            0u8.write(&mut buf0),
            Err(HayesError::InsufficientBuffer {
                required: 1,
                available: 0,
            })
        );

        // 99999 needs 5 bytes
        assert_eq!(
            99999u32.write(&mut buf),
            Err(HayesError::InsufficientBuffer {
                required: 5,
                available: 2,
            })
        );
    }
}
