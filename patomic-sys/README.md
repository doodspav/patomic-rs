# patomic-sys

This crate provides low-level bindings to the `patomic` C library, which can be
found [here](https://github.com/doodspav/patomic).

No documentation is provided, since it would be an unnecessary duplicate of 
existing documentation for the underlying library.

The only tests provided are those which test Rust const functions which replace
C macros.

This crate's version will be kept in sync with the version of the underlying
library.
