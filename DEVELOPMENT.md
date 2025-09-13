# Development

> [!IMPORTANT]
> This project cannot be compiled without the libc2 library. It remains unreleased due to its incomplete state, but a release is planned at a later date.

## Setup

The first step is to install [Rust](https://www.rust-lang.org/tools/install).

Next, to work with Skyline plugins in Rust, install [cargo-skyline](https://github.com/jam1garner/cargo-skyline) by running the following command:

```
cargo install cargo-skyline
```

Lastly, to properly interface with the Nintendo Switch ecosystem through a Skyline plugin, install a fork of the Rust Standard Library by running the following command:

```
cargo skyline update-std
```

## Building

To build a Skyline plugin through cargo-skyline, run the following command:

```
cargo skyline build --release
```

The resulting binary should be located at `./target/aarch64-skyline-switch/release/libstage_config.nro`.

## Formatting

Rust code should be formatted using cargo-fmt, which can be installed by running the following command:

```
rustup component add rustfmt
```

To format Rust code through cargo-fmt, run the following command:

```
cargo fmt
```
