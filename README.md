# stage_config

A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate that enables the use and modification of stage features that are otherwise hardcoded into the game.

The latest version is available from the [Releases](https://github.com/ThatNintendoNerd/stage_config/releases/latest) page.

## Features

Using a configuration file, a stage mod can take advantage of the following features:

- New dynamic ground collisions
- Flatten or unflatten battle objects
- Custom center of gravity or the removal thereof
- Use of `stage_additional_setting` values from spirit battles outside of Spirits
- Discard specialized stage programming
- Guarantee behavior resulting from a particular stage hazards setting

For more information about these features, please read the [wiki](https://github.com/ThatNintendoNerd/stage_config/wiki).

## Building

NOTE: This project cannot be compiled without the libc2 library. Said library is unreleased due to its incomplete state, but its release is planned.

With an up-to-date version of the Rust toolchain installed and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) 3.0.0 or newer, run the following command to compile the project in release mode:

```
cargo skyline build --release
```

The resulting build is found at `./target/aarch64-skyline-switch/release/libstage_config.nro`
