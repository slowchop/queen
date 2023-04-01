# Slowchop Studios Bevy Template

## Todo

* [ ] Turn into bevy-generate template.
* [ ] Handle mouse clicking on menu items.

## Overview

Basic boilerplate for a for making quick prototypes or game jams, with no specific gameplay or graphics in mind.

* Bevy `0.10.0`
* Tweaked compile times:
    * Rust nightly toolchain
    * Dynamic linking
    * LLD linker
* Some test assets
* A game state
* Basic splash screen systems
* Basic menu systems
* Bevy addons:
    * `bevy_egui`
    * `bevy_prototype_debug_lines`

# Assets

* [assets/README.md](assets/README.md) for attribution.

## Ubuntu

```
sudo apt-get install lld
```

## Windows

```
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

# Licence

You may choose to use this code under either of the following licences:

* [MIT](LICENCE-MIT)
* [Apache 2.0](LICENCE-APACHE)

Assets have their own licences, see [assets/README.md](assets/README.md).
