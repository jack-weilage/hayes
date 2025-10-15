use crate::{AtReadable, AtWritable, HayesError};

impl<'at> AtReadable<'at> for bool {
    fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError> {
        if input.is_empty() {
            return Err(HayesError::UnexpectedEnd);
        }

        match input[0] {
            b'0' => Ok((false, 1)),
            b'1' => Ok((true, 1)),
            _ => Err(HayesError::ParseError),
        }
    }
}

impl AtWritable for bool {
    fn write(&self, output: &mut [u8]) -> Result<usize, HayesError> {
        if output.is_empty() {
            return Err(HayesError::InsufficientBuffer {
                required: 1,
                available: 0,
            });
        }

        output[0] = if *self { b'1' } else { b'0' };
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bool() {
        assert_eq!(bool::read(b"0"), Ok((false, 1)));
        assert_eq!(bool::read(b"1"), Ok((true, 1)));
        assert_eq!(bool::read(b"0,"), Ok((false, 1)));
        assert!(bool::read(b"2").is_err());
        assert!(bool::read(b"").is_err());
    }

    #[test]
    fn test_write_bool() {
        let mut buf = [0u8; 10];
        assert_eq!(false.write(&mut buf), Ok(1));
        assert_eq!(&buf[..1], b"0");

        assert_eq!(true.write(&mut buf), Ok(1));
        assert_eq!(&buf[..1], b"1");
    }
}

