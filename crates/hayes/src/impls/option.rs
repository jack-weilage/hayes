use crate::{AtReadable, AtWritable, HayesError};

impl<'at, T: AtReadable<'at>> AtReadable<'at> for Option<T> {
    fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError> {
        // If the input is empty or starts with a delimiter, treat as None
        if input.is_empty() {
            return Ok((None, 0));
        }

        // Try to parse the inner type
        match T::read(input) {
            Ok((value, consumed)) => Ok((Some(value), consumed)),
            // If parsing fails, treat as None with 0 bytes consumed
            // This allows the parser to continue with the next field
            Err(_) => Ok((None, 0)),
        }
    }
}

impl<T: AtWritable> AtWritable for Option<T> {
    fn write(&self, output: &mut [u8]) -> Result<usize, HayesError> {
        match self {
            Some(value) => value.write(output),
            None => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_option_some() {
        assert_eq!(Option::<u8>::read(b"42"), Ok((Some(42), 2)));
        assert_eq!(Option::<u8>::read(b"123,"), Ok((Some(123), 3)));
        assert_eq!(Option::<&str>::read(b"\"hello\""), Ok((Some("hello"), 7)));
    }

    #[test]
    fn test_read_option_none() {
        // Empty input
        assert_eq!(Option::<u8>::read(b""), Ok((None, 0)));

        // Invalid format (not a number)
        assert_eq!(Option::<u8>::read(b"abc"), Ok((None, 0)));

        // Delimiter at start
        assert_eq!(Option::<u8>::read(b",123"), Ok((None, 0)));
    }

    #[test]
    fn test_write_option_some() {
        let mut buf = [0u8; 10];

        assert_eq!(Some(42u8).write(&mut buf), Ok(2));
        assert_eq!(&buf[..2], b"42");

        assert_eq!(Some("test").write(&mut buf), Ok(6));
        assert_eq!(&buf[..6], b"\"test\"");
    }

    #[test]
    fn test_write_option_none() {
        let mut buf = [0u8; 10];

        assert_eq!(Option::<u8>::None.write(&mut buf), Ok(0));
        assert_eq!(Option::<&str>::None.write(&mut buf), Ok(0));
    }
}
