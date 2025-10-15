/// Error type for AT command parsing and generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HayesError {
    /// Buffer does not have sufficient space for the operation
    InsufficientBuffer {
        /// Space required for the operation
        required: usize,
        /// Space available in the buffer
        available: usize,
    },
    /// Failed to parse the input
    ParseError,
    /// Input format is invalid
    InvalidFormat,
    /// Encountered an unsupported error code
    UnsupportedError,
    /// Invalid UTF-8 in string data
    InvalidUtf8,
    /// Unexpected end of input
    UnexpectedEnd,
    /// Modem returned an error
    ModemError,
    /// CME error with code
    CmeError(u16),
    /// CMS error with code
    CmsError(u16),
}

impl core::fmt::Display for HayesError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HayesError::InsufficientBuffer {
                required,
                available,
            } => write!(
                f,
                "Insufficient buffer space: required {required}, available {available}",
            ),
            HayesError::ParseError => write!(f, "Failed to parse input"),
            HayesError::InvalidFormat => write!(f, "Invalid format"),
            HayesError::UnsupportedError => write!(f, "Unsupported error code"),
            HayesError::InvalidUtf8 => write!(f, "Invalid UTF-8 in string data"),
            HayesError::UnexpectedEnd => write!(f, "Unexpected end of input"),
            HayesError::ModemError => write!(f, "Modem returned ERROR"),
            HayesError::CmeError(code) => write!(f, "CME ERROR: {code}"),
            HayesError::CmsError(code) => write!(f, "CMS ERROR: {code}"),
        }
    }
}

impl core::error::Error for HayesError {}
