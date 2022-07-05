# random-xoshiro

Rust to WASM random number generator

## Prerequisites

Get a working rust toolchain. The easiest way is to use [rustup](https://rustup.rs/) and run

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Also need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

You'll also need a working copy of node/npm (we're using 16.x/8.x).

## Build the project

Run

```
wasm-pack build
```

This may take a long time the first time, and will require internet access as it downloads crates.

This will produce a bunch of things, among them `pkg` which holds the js package with typescript definitions, wrapper javascript and the wasm itself.

## Licence

MIT © Imperial College of Science, Technology and Medicine

Please note that this project is released with a [Contributor Code of Conduct](CONDUCT.md). By participating in this project you agree to abide by its terms.
