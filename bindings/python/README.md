# IOTA SDK Library - Python binding

Python binding to the [iota-sdk library](/README.md).

## Table of contents

- [Requirements](#requirements)
- [Getting Started](#getting-started)
    - [Install the IOTA SDK](#install-the-iota-sdk)
- [Client](#client-usage)
- [Wallet](#wallet-usage)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Learn More](#learn-more)

## Requirements

- [Python 3.10+](https://www.python.org)
- [pip ^21.x](https://pypi.org/project/pip)
- `Rust` and `Cargo` to compile the binding. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Getting Started

### Install the IOTA SDK

1. Move to the Python bindings directory:

   ```bash
   cd iota-sdk/bindings/python
   ```

2. (optional) Create a virtual environment and use it. On Linux and macOS, you can run the following commands:

   '''bash
   python3 -m venv iota_sdk_venv
   source iota_sdk_venv/bin/activate
   '''

   If you are using Windows, you should run the following instead:

   ```bash
   .\iota_sdk_venv\Scripts\activate`
   ```

3. Install the required dependencies and build the wheel by running the following commands:

   ```bash
   pip install -r requirements-dev.txt
   pip install .
   ```

4. (optional) If you want to deactivate the virtual environment, run the following command:

   ```bash
   deactivate
   ```

## Client Usage

```python
from iota_sdk import Client

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
```

## Wallet Usage

```python
from iota_sdk import Wallet, StrongholdSecretManager, CoinType

# This example creates a new database and account

wallet_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

secret_manager = StrongholdSecretManager("wallet.stronghold", "some_hopefully_secure_password")

wallet = Wallet('./alice-walletdb', wallet_options, coin_type=CoinType.SHIMMER, secret_manager)

# Store the mnemonic in the Stronghold snapshot. This only needs to be done once
account = wallet.store_mnemonic("flame fever pig forward exact dash body idea link scrub tennis minute " +
          "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

account = wallet.create_account('Alice')
print(account.get_metadata())
```

## Examples

You can use the provided code [examples](examples) to acquainted with the IOTA SDK. You can use the following command to run any example:

```bash
python3 example/[example file]
```

- Where `[example file]` is the file name from the example folder. For example:

```bash
python3 examples/client/00_get_info.py
```

## API Reference

To generate the Python API reference you need to first install the needed dependencies with the following command:

```bash
pip install -r requirements-dev.txt
```

and afterwards you can generate them with:

```bash
PYTHONPATH=. pydoc-markdown
```

## Learn More

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).
