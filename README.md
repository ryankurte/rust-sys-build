# SYS-BUILD

An experimental abstraction over the components required for building rust `*-sys` crates with FFI bindings.

See [rust-embedded#481](https://github.com/rust-embedded/wg/issues/481) for background discussion and [#1](https://github.com/ryankurte/rust-sys-build/issues/1) for a meta-issue.


## Goals

To simplify the construction of `*-sys` crates that adequately support multiple target architectures, minimising the burden on crate authors to create and manage complex `build.rs` configurations and improving the crate consumer experience by providing consistent support for a set of required features.


### Requirements

- Minimise _surprise_ by providing working defaults for common configurations
  - Attempt to use non-interactive discovery mechanisms, provide useful errors, support overrides as required
- Support per-target configuration to pass to underlying build systems
  - ie. existing `autotools` may expect a variety of configuration options per-target
- Support both linking against system libraries and compiling and linking against static or dynamic libraries
- Support cross-platform use 
  - packages should both _work_ and be _cross compilable_ using at least linux / windows / macos
- Provide cargo-centric mechanisms for configuration (no environmental variables required _by default)
- Support fetching pre-compiled libraries / headers for platforms where this is requried / useful
- Support testing of `*-sys` crates across supported hosts and targets


## Status

No more than a rough idea. Check out the [issues](https://github.com/ryankurte/rust-sys-build/issues) or add your own!


## Usage

`sys-build` provides a high-level configuration that allows crate authors to declare how a library should be located, compiled, and linked in their `build.rs`, along with a set of features that can be re-exported from `*-sys` crates to allow higher level crates to choose which of the available mechanisms to use.


## Features

- Locating Sources
  - [ ] `source_dir` enables building from an included directory / submodule
  - [ ] `source_git` enables building from a specified git repository
- Locating Libraries
  - [ ] `use_pkgconfig` enables system library discovery using `pkgconfig`
  - [ ] `use_vcpkg` enable system library discovery
  - [ ] `lib_dir` use default (or provided) library directory
  - [ ] `lib_dl` use a downloaded pre-compiled library
- Binding Generation
  - [ ] `bindgen` enables compile-time binding generation (otherwise included bindings are used)
- Linking mechanisms
  - [ ] `link_static` attempts to statically link the library
  - [ ] `link_dymanic` attempts to dynamically link the libary
- Compilation
  - [ ] `cc` enables compile-time library compilation via the `cc` crate (must be paired with `source_dir` or `source_git`)
  - [ ] `make` enables compile-time library compilation via `make`
  - [ ] `cmake` enables compile-time library compilation via `cmake`
  - [ ] `autotools` enables compile-time library compilation via `autotools`




