<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-foundation/dapps-student-nft" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# Student NFT

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps-student-nft/build.yml?label=Build
[build_href]: https://github.com/gear-foundation/dapps-student-nft/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-foundation/dapps-student-nft/blob/master/LICENSE

[docs_badge]: https://img.shields.io/badge/Docs-online-5023dd
[docs_href]: https://dapp.rs/dapps-student-nft

<!-- Description starts here -->

Special NFT implementation for gear academy students. Learn, react, upvote, comment!

<!-- End of description -->

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-foundation/dapps-student-nft/releases).

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under the [MIT license](LICENSE).
