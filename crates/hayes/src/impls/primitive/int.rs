use crate::{AtReadable, AtWritable, HayesError, impls::BufferWriter};

macro_rules! impl_int {
    ($($ty:ty),*) => {
        $(
            impl<'at> AtReadable<'at> for $ty {
                fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError> {
                    if input.is_empty() {
                        return Err(HayesError::UnexpectedEnd);
                    }

                    // Handle optional negative sign
                    let is_negative = input[0] == b'-';
                    if is_negative && input.len() < 2 {
                        return Err(HayesError::UnexpectedEnd);
                    }

                    let digits = if is_negative { &input[1..] } else { input };

                    // Parse digits
                    // Note: We must use different logic for negative numbers because
                    // the minimum value (e.g., i8::MIN = -128) cannot be represented
                    // as a positive number in the same type (i8::MAX = 127).
                    let mut value: $ty = 0;
                    let mut consumed = 0;

                    if is_negative {
                        for &byte in digits.iter() {
                            if !byte.is_ascii_digit() {
                                break;
                            }

                            #[allow(clippy::cast_lossless, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                            let digit = (byte - b'0') as $ty;
                            value = value
                                .checked_mul(10)
                                .and_then(|v| v.checked_sub(digit))
                                .ok_or(HayesError::ParseError)?;

                            consumed += 1;
                        }
                    } else {
                        for &byte in digits.iter() {
                            if !byte.is_ascii_digit() {
                                break;
                            }

                            #[allow(clippy::cast_lossless, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                            let digit = (byte - b'0') as $ty;
                            value = value
                                .checked_mul(10)
                                .and_then(|v| v.checked_add(digit))
                                .ok_or(HayesError::ParseError)?;

                            consumed += 1;
                        }
                    }

                    if consumed == 0 {
                        return Err(HayesError::ParseError);
                    }

                    if is_negative {
                        consumed += 1;
                    }

                    Ok((value, consumed))
                }
            }

            impl AtWritable for $ty {
                fn write(&self, output: &mut [u8]) -> Result<usize, HayesError> {
                    let available = output.len();

                    let mut writer = BufferWriter::new(output);
                    core::fmt::write(&mut writer, format_args!("{}", self))
                        .map_err(|_| HayesError::InsufficientBuffer {
                            required: if *self == 0 {
                                1
                            } else if *self > 0 {
                                self.unsigned_abs().ilog10() as usize + 1
                            } else {
                                // For negative: need sign + digits
                                1 + self.unsigned_abs().ilog10() as usize + 1
                            },
                            available,
                        })?;

                    Ok(writer.written())
                }
            }
        )*
    };
}

impl_int!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_i8() {
        assert_eq!(i8::read(b"0"), Ok((0, 1)));
        assert_eq!(i8::read(b"123"), Ok((123, 3)));
        assert_eq!(i8::read(b"-42"), Ok((-42, 3)));
        assert_eq!(i8::read(b"-128"), Ok((-128, 4)));
    }

    #[test]
    fn test_write_i8() {
        let mut buf = [0u8; 10];
        assert_eq!(0i8.write(&mut buf), Ok(1));
        assert_eq!(&buf[..1], b"0");

        assert_eq!(123i8.write(&mut buf), Ok(3));
        assert_eq!(&buf[..3], b"123");

        assert_eq!((-42i8).write(&mut buf), Ok(3));
        assert_eq!(&buf[..3], b"-42");
    }

    #[test]
    fn test_write_large_int() {
        let mut buf = [0u8; 50];
        assert_eq!(i64::MAX.write(&mut buf), Ok(19));
        assert_eq!(&buf[..19], b"9223372036854775807");

        assert_eq!(i64::MIN.write(&mut buf), Ok(20));
        assert_eq!(&buf[..20], b"-9223372036854775808");

        assert_eq!(i128::MIN.write(&mut buf), Ok(40));
        assert_eq!(&buf[..40], b"-170141183460469231731687303715884105728");
    }

    #[test]
    fn test_insufficient_buffer() {
        let mut buf = [0u8; 2];

        // 42 needs 2 bytes, buffer has 2 - should succeed
        assert_eq!(42i8.write(&mut buf), Ok(2));

        // 123 needs 3 bytes, buffer has 2 - should fail with required=3
        assert_eq!(
            123i8.write(&mut buf),
            Err(HayesError::InsufficientBuffer {
                required: 3,
                available: 2,
            })
        );

        // -42 needs 3 bytes (sign + 2 digits), buffer has 2 - should fail with required=3
        assert_eq!(
            (-42i8).write(&mut buf),
            Err(HayesError::InsufficientBuffer {
                required: 3,
                available: 2,
            })
        );

        // 0 needs 1 byte
        let mut buf0 = [0u8; 0];
        assert_eq!(
            0i8.write(&mut buf0),
            Err(HayesError::InsufficientBuffer {
                required: 1,
                available: 0,
            })
        );

        // 12345 needs 5 bytes
        assert_eq!(
            12345i32.write(&mut buf),
            Err(HayesError::InsufficientBuffer {
                required: 5,
                available: 2,
            })
        );
    }
}
