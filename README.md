# IOTA SDK Library

[![Coverage Status](https://coveralls.io/repos/github/iotaledger/iota-sdk/badge.svg?branch=develop)](https://coveralls.io/github/iotaledger/iota-sdk?branch=develop)

The IOTA SDK is a Rust-based project that provides a convenient and efficient way to interact with nodes in the
Shimmer and IOTA networks running
the [IOTA 2.0 protocol](https://wiki.iota.org/learn/protocols/iota2.0/introduction-to-digital-autonomy/). It consists of two main
modules: `client` and `wallet`.

## Table of Contents

- [Requirements](#requirements)
  - [Dependencies](#dependencies)
- [Getting Started](#getting-started)
  - [Install the IOTA SDK](#install-the-iota-sdk)
- [Client](#client-usage)
- [Wallet](#wallet-usage)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Contribute](#contribute)
- [License](#license)

## Features

- **Client module**: The `client` module in the IOTA SDK offers low-level functions that allow you to have
  fine-grained control over your interactions with Shimmer nodes. The module is stateless. It provides access to the
  underlying API endpoints and enables advanced operations such as custom message construction and direct communication
  with the network.

- **Wallet module**: The `wallet` module in the IOTA SDK provides high-level functions for managing accounts, generating
  addresses, creating transactions, and interacting with the Shimmer network. It offers a user-friendly interface for
  developers to build applications on the Shimmer network. It is stateful, and it can optionally interact
  with [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs/) for seed handling, storage, and state backup.

- **Bindings**: The IOTA SDK includes bindings for `Python`, `Node.js`, and `WASM`, which allow you
  to use the SDK in your preferred programming language. These bindings provide seamless integration with existing
  projects, enabling cross-platform compatibility and flexibility.

## Branching Structure for Development

This library follows the following branching strategy:

| Branch       | Description                                                                                                                    |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| `develop`    | Ongoing development for future releases of the staging networks. This branch gets merged into `staging` on release.            |
| `production` | The latest releases for the IOTA network.                                                                                      |
| `staging`    | The latest releases for the Shimmer network.                                                                                   |
| other        | Other branches that may reflect current projects. Like `develop`, they will find their way into `staging` once they are ready. |

## Before You Start

This file is focused on the Rust core SDK. Please refer to
the [Python](bindings/python/README.md), [Node.js](bindings/nodejs/README.md) and [Wasm](bindings/wasm/README.md)
instructions if you want information on installing and using them.

## Requirements

The IOTA SDK requires `Rust` and `Cargo`. You can find installation instructions in
the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

We recommend that you update the Rust compiler to the latest stable version first:

```shell
rustup update stable
```

### Dependencies

You must also install `cmake`, `clang`, and `openssl`. You may need to install additional build tools on your system to
run the build process successfully using Cargo.

#### Windows

You can download `cmake` from the [official website](https://cmake.org/download/). You can install `openssl`
with [vcpkg](https://github.com/microsoft/vcpkg) or [chocolatey](https://chocolatey.org/).

- Installing `openssl` with `vcpkg`:

```
./vcpkg.exe install openssl:x64-windows
./vcpkg.exe integrate install
# You may want to add this to the system environment variables since you'll need it to compile the crate
set VCPKGRS_DYNAMIC=1
```

- Installing `openssl` with `chocolatey`:

```
choco install openssl
# You may need to set the OPENSSL_DIR environment variable
set OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
```

#### macOS

You can install `cmake` and `openssl` with [`Homebrew`](https://brew.sh/):

```
brew install cmake openssl@1.1
```

#### Linux

You can install `cmake`, `clang`, and `openssl` with your distro's package manager or download them from their websites.
On Debian and Ubuntu, you will also need the `build-essential` and `libudev-dev` packages.

## Getting Started

### Install the IOTA SDK

To start using the IOTA SDK in your Rust project, you can include the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
iota-sdk = { git = "https://github.com/iotaledger/iota-sdk", branch = "develop" }
```

## Client Usage

The following example creates a Client instance connected to the Shimmer Testnet, and retrieves the node's information by calling `Client.get_info()`, and then print the node's information.

[sdk/examples/client/getting_started.rs](sdk/examples/client/getting_started.rs)

## Wallet Usage

The following example will create a new Wallet Account using a StrongholdSecretManager. For this `features = ["stronghold"]` is needed in the Cargo.toml import. To persist the wallet in a database, `"rocksdb"` can be added.

[sdk/examples/wallet/getting_started.rs](sdk/examples/wallet/getting_started.rs)

## Examples

You can use the provided code [examples](sdk/examples) to get acquainted with the IOTA SDK. You can use the following
command to run any example:

```bash
cargo run --release --all-features --example example_name
```

Where `example_name` is the name from the [Cargo.toml](sdk/Cargo.toml) name from the example folder. For example:

```bash
cargo run --release --all-features --example create_account
```

You can get a list of the available code examples with the following command:

```bash
cargo run --example
```

## API Reference

You can find the IOTA SDK Rust API Reference is in
the [IOTA SDK crate documentation](https://docs.rs/iota-sdk/latest/iota_sdk/).

## Contribute

If you find any issues or have suggestions for improvements,
please [open an issue](https://github.com/iotaledger/iota-sdk/issues/new/choose) on the GitHub repository. You can also
submit [pull requests](https://github.com/iotaledger/iota-sdk/compare)
with [bug fixes](https://github.com/iotaledger/iota-sdk/issues/new?assignees=&labels=bug+report&projects=&template=bug_report.yml&title=%5BBug%5D%3A+),
[new features](https://github.com/iotaledger/iota-sdk/issues/new?assignees=&labels=&projects=&template=feature_request.md),
or documentation enhancements.

Before contributing, please read and adhere to the [code of conduct](/.github/CODE_OF_CONDUCT.md).

## License

The IOTA SDK is open-source software licensed under Apache License 2.0. For more information, please read
the [LICENSE](/LICENSE).
