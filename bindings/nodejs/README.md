# IOTA SDK Library - Node.js binding

## Table of contents

- [IOTA SDK Library - Node.js binding](#iota-sdk-library---nodejs-binding)
  - [Table of contents](#table-of-contents)
  - [Requirements](#requirements)
    - [Windows](#windows)
  - [Getting Started](#getting-started)
    - [Installation Using a Package Manager](#installation-using-a-package-manager)
      - [npm](#npm)
      - [Yarn](#yarn)
    - [Install the Binding from Source](#install-the-binding-from-source)
      - [Build nodejs bindings](#build-nodejs-bindings)
  - [Client Usage](#client-usage)
  - [Wallet Usage](#wallet-usage)
  - [Examples](#examples)
  - [API Reference](#api-reference)
  - [Available Scripts](#available-scripts)
    - [`npm install` or `yarn install`](#npm-install-or-yarn-install)
    - [`npm run build` or `yarn build`](#npm-run-build-or-yarn-build)
    - [`npm run test` or `yarn test`](#npm-run-test-or-yarn-test)
    - [`npm run create-api-docs` or `yarn create-api-docs`](#npm-run-create-api-docs-or-yarn-create-api-docs)
  - [Important Files and Directories](#important-files-and-directories)
  - [Learn More](#learn-more)

## Requirements

These are requirements for building the binary.

Please ensure you have installed the [required dependencies for the library for Rust code](/README.md#requirements), as
well as the following:

- Python < 3.11
- Yarn v1

### Windows

On Windows, you will also need an LLVM. Our workflow uses
`https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.6/LLVM-16.0.6-win64.exe`. You may also need to set
an environment variable `RUSTFLAGS` to `-C target-feature=+crt-static`.

## Getting Started

### Installation Using a Package Manager

To install the library from your package manager of choice, you only need to run the following:

#### npm

```sh
npm i @iota/sdk
```

#### Yarn:

```sh
yarn add @iota/sdk
```

### Install the Binding from Source

Installing the Node.js bindings requires
a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

This will guide you in any dependencies and running the build.

#### Build nodejs bindings

If you have already installed the project and only want to run the build, run the following:

```sh
npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust
build and copy the built library into `./build/Release/index.node`.
Prebuild requires that the binary is in `build/Release` as though it was built with node-gyp.

## Client Usage

The following example creates a [`Client`](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/classes/Client/)
instance connected to
the [Shimmer Testnet](https://api.testnet.shimmer.network), and retrieves the node's information by
calling [`Client.getInfo()`](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/classes/Client/#getinfo),
and then print the node's information.

```javascript
const { Client, initLogger } = require('@iota/sdk');

async function run() {
    initLogger();

    const client = new Client({
        nodes: ['https://api.testnet.shimmer.network'],
        localPow: true,
    });

    try {
        const nodeInfo = await client.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
```

## Wallet Usage

The following example will create a
new [`Wallet`](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/classes/Wallet/) [`Account`](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/classes/Account/)
that connects to the [Shimmer Testnet](https://api.testnet.shimmer.network) using the
[`StrongholdSecretManager`](https://wiki.iota.org/shimmer/iota-sdk/references/python/iota_sdk/secret_manager/#strongholdsecretmanager-objects).

```javascript
import {  Wallet, CoinType, WalletOptions } from '@iota/sdk';

const walletOptions: WalletOptions = {
    storagePath: `Alice`, // A name to associate with the created account.
    clientOptions: {
        nodes: ['https://api.testnet.shimmer.network'], // The node to connect to.
    },
    coinType: CoinType.Shimmer,
    secretManager: {
        // Setup Stronghold secret manager
        stronghold: {
            snapshotPath: 'vault.stronghold', //  The path to store the account snapshot.
            password: 'a-secure-password', // A password to encrypt the stored data. WARNING: Never hardcode passwords in production code.
        },
    },
};
const wallet = new Wallet(walletOptions);
```

## Examples

You can use the provided code [examples](https://github.com/iotaledger/iota-sdk/tree/develop/bindings/nodejs/examples) to get acquainted with the IOTA SDK. You can use the following
command to run any example:

```bash
cd examples
yarn run-example ./[example folder]/[example file]
```

- Where `[example file]` is the file name from the example folder. For example:

```bash
node examples/client/00_get_info.ts
```

## API Reference

You can find the API reference for the Node.js bindings in the
[IOTA Wiki](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/api_ref/).

## Available Scripts

In the project directory, you can run the following:

### `npm install` or `yarn install`

Installs the project, including running `npm run build`.

### `npm run build` or `yarn build`

Builds the Node addon (`index.node`) from source.

### `npm run test` or `yarn test`

Runs the unit tests by calling `cargo test`. You can learn more
about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from
the [Rust book](https://doc.rust-lang.org/book/).

### `npm run create-api-docs` or `yarn create-api-docs`

This is mainly just used to create the API docs in the [Wiki](https://github.com/iota-wiki/iota-wiki). Executed locally it will generate a `docs` folder in the current working directory with the API docs.

## Important Files and Directories

- `Cargo.toml`

  The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html) informs the `cargo` command.

- `index.node`

  The Node addon - i.e., a binary Node module - is generated by building the project. This is the main module for this
  package,
  as dictated by the `"main"` key in `package.json`.

  Under the hood, a [Node addon](https://nodejs.org/api/addons.html) is
  a [dynamically-linked shared object](<https://en.wikipedia.org/wiki/Library_(computing)#Shared_libraries>).
  The `"build"`
  script produces this file by copying it from within the `target/` directory, which is where the Rust build produces
  the shared object.

- `package.json`

  The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.

- `src/`

  The directory tree that contains the Rust source code for the project.

- `src/lib.rs`

  The Rust library's main module.

- `target/`

  Binary artifacts generated by the Rust build.

## Learn More

To learn more about Neon, see the [Neon documentation](https://neon-bindings.com).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).
