# Rust Gerber Library

[![Build status][build-status-badge]][build-status]
[![Crates.io][crates-io-badge]][crates-io]

- [Docs (released)](https://docs.rs/gerber-types/)

This crate implements the basic building blocks of Gerber X2 (compatible with
Gerber RS-274X) code. It focusses on the low level types (to be used like an
AST) and code generation and does not do any semantic checking.

For example, you can use an aperture without defining it. This will generate
syntactically valid but semantially invalid Gerber code, but this module won't
complain.

The plan is to write a high-level wrapper library on top of this. Early drafts
[are in progress](https://github.com/dbrgn/gerber-rs) but the design isn't
fixed yet.

Minimal required Rust version: 1.31.

Current Gerber X2 spec: https://www.ucamco.com/files/downloads/file/81/the_gerber_file_format_specification.pdf

## Example

You can find an example in the [`examples`
directory](https://github.com/dbrgn/gerber-types-rs/blob/main/examples/polarities-apertures.rs).
It's still quite verbose, the goal is to make the API a bit more ergonomic in
the future. (This library has a low-level focus though, so it will never get a
high-level API. That is the task of other libraries.)

To generate Gerber code for that example:

    $ cargo run --example polarities-apertures

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- Badges -->
[build-status]: https://github.com/dbrgn/gerber-types-rs/actions?query=workflow%3ACI
[build-status-badge]: https://img.shields.io/github/workflow/status/dbrgn/gerber-types-rs/CI/main
[crates-io]: https://crates.io/crates/gerber-types
[crates-io-badge]: https://img.shields.io/crates/v/gerber-types.svg
