[![CI](https://github.com/pkgcraft/pkgcraft-c/workflows/CI/badge.svg)](https://github.com/pkgcraft/pkgcraft-c/actions/workflows/ci.yml)

# pkgcraft-c

C bindings for pkgcraft.

## Development

Requirements: [cargo-c](https://crates.io/crates/cargo-c), meson (to build and
run the tests), and everything required to build pkgcraft

Use the following commands to set up a dev environment:

```bash
# clone the pkgcraft workspace
git clone --recurse-submodules https://github.com/pkgcraft/pkgcraft-workspace.git
cd pkgcraft-workspace

# build the C library
source ./build pkgcraft-c

# build and run the tests
meson setup target/build pkgcraft-c && meson test -C target/build
```
