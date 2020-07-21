# SYS-BUILD

An experimental abstraction over the components required for building rust `*-sys` crates with FFI bindings.

See [rust-embedded#481](https://github.com/rust-embedded/wg/issues/481) for background discussion.

## Goals

To simplify the construction of `*-sys` crates that adequately support multiple target architectures, minimising the burden on crate authors to create and manage complex `build.rs` configurations and improving the crate consumer experience by providing consistent support for a set of required features.

## Status

No more than a rough idea. Check out the [issues](https://github.com/ryankurte/rust-sys-build/issues) or add your own!

## Usage

`sys-build` provides a high-level configuration that allows crate authors to declare how a library should be located, compiled, and linked, along with a set of features that can be re-exported from `*-sys` crates to allow higher level crates to choose which mechanisms to use.


