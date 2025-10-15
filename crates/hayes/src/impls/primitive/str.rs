use crate::{AtReadable, AtWritable, HayesError};

impl<'at> AtReadable<'at> for &'at str {
    fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError> {
        if input.is_empty() {
            return Err(HayesError::UnexpectedEnd);
        }

        // Expect opening quote
        if input[0] != b'"' {
            return Err(HayesError::InvalidFormat);
        }

        // Find closing quote
        let closing_quote_pos = input[1..]
            .iter()
            .position(|&b| b == b'"')
            .ok_or(HayesError::UnexpectedEnd)?;

        // Extract string between quotes
        let str_bytes = &input[1..=closing_quote_pos];
        let s = core::str::from_utf8(str_bytes).map_err(|_| HayesError::InvalidUtf8)?;

        // Return string and total bytes consumed (opening quote + string + closing quote)
        Ok((s, closing_quote_pos + 2))
    }
}

impl AtWritable for &str {
    fn write(&self, output: &mut [u8]) -> Result<usize, HayesError> {
        let required = self.len() + 2; // +2 for quotes

        if output.len() < required {
            return Err(HayesError::InsufficientBuffer {
                required,
                available: output.len(),
            });
        }

        output[0] = b'"';
        output[1..=self.len()].copy_from_slice(self.as_bytes());
        output[1 + self.len()] = b'"';

        Ok(required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_str() {
        assert_eq!(<&str>::read(b"\"hello\""), Ok(("hello", 7)));
        assert_eq!(<&str>::read(b"\"\""), Ok(("", 2)));
        assert_eq!(<&str>::read(b"\"test\","), Ok(("test", 6)));
        assert!(<&str>::read(b"hello").is_err()); // No quotes
        assert!(<&str>::read(b"\"hello").is_err()); // No closing quote
    }

    #[test]
    fn test_write_str() {
        let mut buf = [0u8; 20];
        assert_eq!("hello".write(&mut buf), Ok(7));
        assert_eq!(&buf[..7], b"\"hello\"");

        assert_eq!("".write(&mut buf), Ok(2));
        assert_eq!(&buf[..2], b"\"\"");

        // Test insufficient buffer
        let mut small_buf = [0u8; 3];
        assert!("hello".write(&mut small_buf).is_err());
    }
}
