# Rust Gerber Library

This crate implements the basic building blocks of Gerber code. It focusses on
the low level types and does not do any semantic checking.

For example, you can use an aperture without defining it. This will generate
syntactically valid but semantially invalid Gerber code, but this module won't
complain.

Gerber spec: https://www.ucamco.com/files/downloads/file/81/the_gerber_file_format_specification.pdf
