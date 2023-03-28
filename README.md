# CoinUtils-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

CoinUtils-src crate is a *-src crate. This links CoinUtils libraries to executable build by cargo, but does not provide Rust bindings.

By this package, you don't need to worry about installing CoinUtils in the system, and it's a package for **all platforms**.

CoinUtils is an open-source collection of classes and helper functions that are generally useful to multiple COIN-OR projects.

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
coinutils-src = "0.2"
```

This package does not provide bindings. Please use [coincbc-sys], [coinclp-sys] to use Cbc, Clp, e.g.

```toml
[dependencies]
coincbc-sys = { version = "0.2" }
```

## Configuration

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `coinutils-src` will use 
[vcpkg] to find CoinUtils. Before building, you must have the correct CoinUtils 
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `coinutils` for the `x64-windows` triplet:

```sh
vcpkg install coinutils --triplet x64-windows
```

To link CoinUtils statically, install `coinutils` for the `x64-windows-static-md` triplet:

```sh
vcpkg install coinutils --triplet x64-windows-static-md
```

To link CoinUtils and C Runtime (CRT) statically, install `coinutils` for the `x64-windows-static` triplet:

```sh
vcpkg install coinutils --triplet x64-windows-static
```

and build with `+crt-static` option

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can compile it for the other target by providing the `--target` option to 
`cargo build`. 


| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-gnu`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils

[coincbc-sys]: https://github.com/Maroon502/coincbc-sys
[coinclp-sys]: https://github.com/Maroon502/coinclp-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/coinutils-src/badge.svg
[documentation-url]: https://docs.rs/coinutils-src
[package-img]: https://img.shields.io/crates/v/coinutils-src.svg
[package-url]: https://crates.io/crates/coinutils-src
[license-img]: https://img.shields.io/crates/l/coinutils-src.svg
[license-url]: https://github.com/Maroon502/coinutils-src/blob/master/LICENSE.md