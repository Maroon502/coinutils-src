# CoinUtils-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

The package provides a source of [CoinUtils].

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
coinutils-src = "0.1"
```

## Configuration

The following Cargo features are supported:

* `static` to link to CoinUtils statically;
* `system` to skip building the bundled CoinUtils.

The following environment variables are supported:

* `COINUTILS_STATIC` to link to CoinUtils statically;

## Windows and vcpkg

On Windows, `coinutils-src` relies on [vcpkg] to find CoinUtils. Before building,
you must have the correct CoinUtils installed for your target triplet and kind of
linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc`
toolchain, install `coinutils` for the `x64-windows` triplet:

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
[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/coinutils-src/badge.svg
[documentation-url]: https://docs.rs/coinutils-src
[package-img]: https://img.shields.io/crates/v/coinutils-src.svg
[package-url]: https://crates.io/crates/coinutils-src
[license-img]: https://img.shields.io/crates/l/coinutils-src.svg
[license-url]: https://github.com/Maroon502/coinutils-src/blob/master/LICENSE.md