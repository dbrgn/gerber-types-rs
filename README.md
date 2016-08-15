# Rust Gerber Library

[![Travis CI][travis-ci-badge]][travis-ci]
[![Coveralls][coveralls-badge]][coveralls]
[![Crates.io][crates-io-badge]][crates-io]

[Docs](https://dbrgn.github.io/gerber-types-rs/)

This crate implements the basic building blocks of Gerber code. It focusses on
the low level types and does not do any semantic checking.

For example, you can use an aperture without defining it. This will generate
syntactically valid but semantially invalid Gerber code, but this module won't
complain.

Minimal required Rust version: 1.3.

Gerber spec: https://www.ucamco.com/files/downloads/file/81/the_gerber_file_format_specification.pdf

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- Badges -->
[travis-ci]: https://travis-ci.org/dbrgn/gerber-types-rs
[travis-ci-badge]: https://img.shields.io/travis/dbrgn/gerber-types-rs.svg
[coveralls]: https://coveralls.io/github/dbrgn/gerber-types-rs
[coveralls-badge]: https://img.shields.io/coveralls/dbrgn/gerber-types-rs.svg
[crates-io]: https://crates.io/crates/gerber-types
[crates-io-badge]: https://img.shields.io/crates/v/gerber-types.svg
