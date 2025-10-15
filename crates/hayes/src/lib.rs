#![no_std]
#![doc = include_str!("../README.md")]

//! ## Manual Implementation Example
//!
//! Without the derive macros, you can manually implement the traits.
//! See the [`manual_impl` example](https://github.com/jack-weilage/hayes/blob/main/crates/hayes/examples/manual_impl.rs)
//! for a comprehensive demonstration of how to:
//!
//! - Implement `AtCommand` for query commands (e.g., `AT+CFUN?`)
//! - Implement `AtCommand` for set commands with parameters (e.g., `AT+CFUN=1`)
//! - Implement `AtResponse` for parsing responses with single or multiple fields
//! - Handle optional parameters and complex response formats
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use hayes::{AtCommand, AtResponse, AtReadable, AtWritable, HayesError};
//!
//! // Define a command
//! struct ReadFunctionality;
//!
//! impl AtCommand for ReadFunctionality {
//!     type Response<'at> = FunctionalityResponse;
//!
//!     fn write(&self, buffer: &mut [u8]) -> Result<usize, HayesError> {
//!         let cmd = b"AT+CFUN?\r\n";
//!         if buffer.len() < cmd.len() {
//!             return Err(HayesError::InsufficientBuffer {
//!                 required: cmd.len(),
//!                 available: buffer.len(),
//!             });
//!         }
//!         buffer[..cmd.len()].copy_from_slice(cmd);
//!         Ok(cmd.len())
//!     }
//! }
//! ```

#![deny(clippy::cargo, missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

mod error;
mod impls;

pub use error::HayesError;

// #[cfg(feature = "derive")]
// pub use hayes_derive::{Command, Response};

/// Trait for types that can be read from AT command/response buffers
///
/// The `'at` lifetime ties the parsed value to the input buffer, enabling
/// zero-copy parsing for borrowed types like `&'at str`.
pub trait AtReadable<'at>: Sized {
    /// Read a value from the input buffer
    ///
    /// Returns the parsed value and the number of bytes consumed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The buffer doesn't contain enough data
    /// - The data format is invalid
    /// - The data cannot be parsed into the target type
    fn read(input: &'at [u8]) -> Result<(Self, usize), HayesError>;
}

/// Trait for types that can be written to AT command/response buffers
pub trait AtWritable {
    /// Write a value to the output buffer
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The buffer doesn't have sufficient space
    /// - The value cannot be formatted
    fn write(&self, output: &mut [u8]) -> Result<usize, HayesError>;
}

/// Trait for AT commands
///
/// Types implementing this trait represent AT commands that can be sent to a modem.
pub trait AtCommand {
    /// The response type associated with this command
    type Response<'at>: AtResponse<'at>;

    /// Write the command to the output buffer
    ///
    /// The output will be formatted as `AT<command>\r\n`.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer doesn't have sufficient space.
    fn write(&self, buffer: &mut [u8]) -> Result<usize, HayesError>;
}

/// Trait for AT responses
///
/// Types implementing this trait represent responses from a modem.
/// The `'at` lifetime enables zero-copy parsing of response fields.
pub trait AtResponse<'at>: Sized {
    /// Read a response from the input buffer
    ///
    /// This method handles the full response including status codes like
    /// `OK`, `ERROR`, `+CME ERROR: <code>`, and `+CMS ERROR: <code>`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The buffer doesn't contain a complete response
    /// - The response format is invalid
    /// - The modem returned an error
    fn read(buffer: &'at [u8]) -> Result<Self, HayesError>;
}
