#![no_std]

use hayes::{Command, Response};

/// Read the current functionality mode.
#[derive(Command)]
#[at("+CFUN?", FunctionalityResponse)]
struct ReadFunctionality;

#[derive(Response)]
#[at("+CFUN")]
struct FunctionalityResponse {
    mode: FunctionalityMode,
}

#[derive(Command)]
#[at("+CFUN")]
struct SetFunctionality {
    mode: FunctionalityMode,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum FunctionalityMode {
    Minimum = 0,
    Full = 1,
    DisableTransmitReceive = 4,
}

fn main() {
    let mut tx_buffer = [0u8; 64];
    let mut rx_buffer = [0u8; 64];

    let len = ReadFunctionality
        .write(&mut tx_buffer)
        .expect("Failed to write AT command");
    assert_eq!(&tx_buffer[..len], b"AT+CFUN?\r\n");

    // Somewhere in here, your code should send tx_buffer to the modem and receive into rx_buffer.

    // The type of `ReadFunctionality::Response` will be `FunctionalityResponse`, so this could
    // be `T::Response::read` in a custom `send_at` function.
    let response = FunctionalityResponse::read(&rx_buffer[..]).expect("Failed to read AT response");
    assert_eq!(response.mode, FunctionalityMode::Full);

    let len = SetFunctionality {
        mode: FunctionalityMode::Minimum,
    }
    .write(&mut tx_buffer)
    .expect("Failed to write AT command");
    assert_eq!(&tx_buffer[..len], b"AT+CFUN=0\r\n");
}
