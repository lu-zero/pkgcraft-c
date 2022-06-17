[![CI](https://github.com/pkgcraft/pkgcraft-c/workflows/CI/badge.svg)](https://github.com/pkgcraft/pkgcraft-c/actions/workflows/ci.yml)

# pkgcraft-c

C bindings for pkgcraft.

## Development

Requirements: [cargo-c](https://crates.io/crates/cargo-c) and everything required to build pkgcraft

Use the following commands to set up a dev environment:

```bash
# clone the pkgcraft workspace
git clone --recursive-submodules https://github.com/pkgcraft/pkgcraft-workspace.git
cd pkgcraft-workspace

# build the C library
./build-c-lib
```
