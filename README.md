# iota-sdk

[![Coverage Status](https://coveralls.io/repos/github/iotaledger/iota-sdk/badge.svg?branch=develop)](https://coveralls.io/github/iotaledger/iota-sdk?branch=develop)

The iota-sdk is a Rust-based project that provides a convenient and efficient way to interact with Shimmer nodes in the
Shimmer network. It consists of two main modules: `wallet` and `client`.

The `wallet` module is stateful, with a standardized interface for developers to build applications involving value
transactions. It uses high-level functions that simplify everyday operations. It can optionally interact
with [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs/) for seed handling and storage and state backup.

The `client` module is stateless. It aims to provide more flexibility and access to low-level functions.

## Table of Contents

- [Features](#features)
- [Branching Structure for Development](#branching-structure-for-development)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Dependencies](#dependencies)
    - [Linux](#linux)
- [Install the iota-sdk](#install-the-iota-sdk)
    - [Rust](#rust)
    - [Node.js](#nodejs)
    - [Python](#python)
- [Usage](#usage)
    - [Wallet](#wallet)
    - [Client](#client)
    - [API Reference](#api-reference)
    - [Usage Examples](#usage-examples)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

- **Wallet module**: The `wallet` module in the iota-sdk provides high-level functions for managing accounts, generating
  addresses, creating transactions, and interacting with the Shimmer network. It offers a user-friendly interface for
  developers to build applications on the Shimmer network.

- **Client module**: The `client` module in the iota-sdk offers low-level functions that allow you to have
  fine-grained control over their interactions with Shimmer nodes. It provides access to the underlying API endpoints
  and enables advanced operations such as custom message construction and direct communication with the network.

- **Python and Node.js Bindings**: The iota-sdk includes bindings for Python and Node.js, which allow you to use the
  SDK in your preferred programming language. These bindings provide seamless integration with existing Python and
  Node.js projects, enabling cross-platform compatibility and flexibility.

## Branching Structure for Development

This library follows the following branching strategy:

| Branch       | Description                                                                                                                    |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| `develop`    | Ongoing development for future releases of the networks. This branch gets merged into `staging` on releases.                   |
| `production` | The latest releases for the IOTA network.                                                                                      |
| `staging`    | The latest releases for the Shimmer network.                                                                                   |
| other        | Other branches that may reflect current projects. Like `develop`, they will find their way into `staging` once they are ready. |

## Getting Started

### Prerequisites

The iota-sdk requires `Rust` and `Cargo`. You can find installation instructions in
the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

We recommend you first update the Rust compiler to the latest stable version:

```shell
rustup update stable
```

The nightly Rust compiler should be fine, but some changes might not be compatible.

### Dependencies

You must also install `cmake`, `clang`, and `openssl`. You may need to install additional build tools on your system to run the build process successfully using Cargo.

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

You can install `cmake` and `openssl` with `Homebrew`:

```
brew install cmake openssl@1.1
```

### Linux

You can install `cmake`, `clang`, and `openssl` with your distro's package manager or download them from their websites. On
Debian and Ubuntu, you will also need the `build-essential` and `libudev-dev` packages.

## Install the iota-sdk

### Rust

To start using the iota-sdk in your Rust project, you can include the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
iota-sdk = " { git = "https://github.com/iotaledger/iota-sdk", branch = "develop" }"
```

### Node.js

```bash
npm i @iota/sdk
```

### Python

For Python and Node.js projects, you can install the SDK using the respective package managers:

```bash
pip install iota-sdk
```

## Usage

### Wallet

To use the wallet, you need to create a `Wallet`:

#### Rust

```rust
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{stronghold::StrongholdSecretManager, SecretManager},
    },
    wallet::{ClientOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup Stronghold secret manager.
    let secret_manager = StrongholdSecretManager::builder()
        .password("vault.stronghold") // A password to encrypt the stored data.WARNING: Never hardcode passwords in production code.
        .build(PathBuf::from("vault.stronghold"))?; // The path to store the account snapshot.

    let client_options = ClientOptions::new().with_node("https://api.testnet.shimmer.network")?;// The node to connect to.

    // Set up and store the wallet.
    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Stronghold(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Generate a mnemonic and store it in the Stronghold vault.
    // INFO: It is best practice to back up the mnemonic somewhere secure.
    let mnemonic = wallet.generate_mnemonic()?;
    wallet.store_mnemonic(mnemonic).await?;

    // Create an account.
    let account = wallet
        .create_account()
        .with_alias("Alice".to_string()) // A name to associate with the created account.
        .finish()
        .await?;


    Ok(())
}
```

#### Node.js

```nodejs
import { initLogger, Wallet, CoinType, WalletOptions } from '@iota/sdk';

const walletOptions: WalletOptions = {
    storagePath: `Alice`,// A name to associate with the created account.
    clientOptions: {
        nodes: ['https://api.testnet.shimmer.network'],// The node to connect to.
    },
    coinType: CoinType.Shimmer,
    secretManager: { // Setup Stronghold secret manager
        stronghold: {
            snapshotPath: 'vault.stronghold',//  The path to store the account snapshot.
            password:  'a-secure-password',// A password to encrypt the stored data. WARNING: Never hardcode passwords in production code.
        },
    },
};
const wallet = new Wallet(walletOptions);
```

#### Python

```python
from iota_sdk import Wallet, StrongholdSecretManager, CoinType, Utils

# A name to associate with the created account.
ACCOUNT_ALIAS = 'Alice'

# The node to connect to.
node_url = 'https://api.testnet.shimmer.network'

# A password to encrypt the stored data.
# WARNING: Never hardcode passwords in production code.
STRONGHOLD_PASSWORD = 'a-secure-password'

# The path to store the account snapshot.
STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold'

# Setup Stronghold secret manager
secret_manager = StrongholdSecretManager(STRONGHOLD_SNAPSHOT_PATH, STRONGHOLD_PASSWORD)

# Set up the client options
client_options = {
    'nodes': [node_url],
}

# Set up and store the wallet.
wallet = Wallet(
    client_options=client_options,
    coin_type=CoinType.SHIMMER,
    secret_manager=secret_manager
)
```

### Client

To use the client, you simply need to create a `Client`.

#### Rust

```rust
use iota_sdk::client::{
    Client,
};

let client = Client::builder()
    .with_node('https://api.testnet.shimmer.network')? // Insert your node URL here
    .finish()
    .await?;
```

#### Node.js

```nodejs
import { Client } from '@iota/sdk';

const client = new Client({nodes: "https://api.testnet.shimmer.network/"});
```

#### Python

```python
from iota_sdk import Client

client = Client('https://api.testnet.shimmer.network')// Insert your node URL here
```

### API Reference

#### Rust

The iota-sdk Rust API Reference is in the [crate documentation](https://docs.rs/iota-sdk/latest/iota_sdk/).

#### Python

You can find the
iota-sdk [Python API Reference in the IOTA Wiki](https://wiki.iota.org/next/iota-sdk/references/nodejs/api_ref/).

#### Node.js

You can find the
iota-sdk [Node.js API Reference in the IOTA Wiki](https://wiki.iota.org/next/iota-sdk/references/python/iota_wallet/account/).

### Usage Examples

You can find more usage examples in the [IOTA Wiki](https://wiki.iota.org/shimmer/iota-sdk/how_tos/run_how_tos/), and in
the repositories example folder for each language:

- [Rust](https://github.com/iotaledger/iota-sdk/tree/develop/sdk/examples).
- [Python](https://github.com/iotaledger/iota-sdk/tree/develop/bindings/python/examples)
- [Node.js](https://github.com/iotaledger/iota-sdk/tree/develop/bindings/nodejs/examples)

## Contributing

If you find any issues or have suggestions for improvements, please open an issue on the GitHub repository. You can also
submit pull requests with bug fixes, new features, or documentation enhancements.

Before contributing, please read and adhere to the [code of conduct](/.github/CODE_OF_CONDUCT.md).

## License

The iota-sdk is open-source software licensed under Apache License 2.0. For more information, please read
the [LICENSE.md](/.github/LICENSE.md)

## Acknowledgments

The iota-sdk project is built upon the contributions of many individuals and organizations. We would like to express our
gratitude to the IOTA community and all the developers who have contributed to the project.

For a complete list of contributors and acknowledgments, please refer to
the [GitHub repository's contributor page](https://github.com/iotaledger/iota-sdk/graphs/contributors).
