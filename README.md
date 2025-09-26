# `hayes`

A **`#![no_std]`**, **no_alloc** library for serializing and deserializing Hayes commands, more commonly known as AT commands.

`hayes` allows you to easily define your own AT command sets using Rust's type system, and provides a simple API for encoding and decoding these commands.

```rust
use hayes::{Command, Response};

/// Defining an AT command is natural: Just create a struct with the inputs (or none at all)!
/// Include the response type (the second parameter) if necessary.
#[derive(Command)]
#[at("+CFUN?", FunctionalityMode)]
struct GetFunctionality;

/// Defining the response for a command works the exact same!
#[derive(Response)]
#[at("+CFUN")]
struct FunctionalityResponse {
    mode: FunctionalityMode
}

/// Enums work as you'd expect! No special syntax or definitions required.
#[repr(u8)]
enum FunctionalityMode {
    Minimum = 0,
    Full = 1,
    DisableTransmitReceive = 4,
}
```

## Examples

Check out the [examples](./examples) for more in-depth, worked usage.

## Feature Flags

- **defmt**: Derives `defmt::Format` on exported structs and enums.
- **derive**: Re-exports the `Command` and `Response` derive macros from [`hayes_derive`](./crates/hayes_derive) for simpler implementations.

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `hayes` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
