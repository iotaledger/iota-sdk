# IOTA SDK Library - Python binding

Python binding to the iota-sdk library.

## Requirements

[Python 3.x](https://www.python.org) & [pip](https://pypi.org/project/pip)

`Rust` and `Cargo`, to compile the binding. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Installation

- Go to `iota-sdk/bindings/python`

### Create a virtual environment and use it (optional)
- `python3 -m venv iota_sdk_venv`
- `source iota_sdk_venv/bin/activate`; Windows: `.\iota_sdk_venv\Scripts\activate`

### Install required dependencies and build the wheel

- `pip install -r requirements-dev.txt`
- `pip install .`

### Run examples

`python3 example/[example file]`

Example: 
- `python3 examples/client/00_get_info.py`

### To deactivate the virtual environment (optional)

- `deactivate`

## Getting Started

After you installed the library, you can create a `Client` instance and interface with it.

```python
from iota_sdk import Client

# Create a Client instance
client = Client(nodes = ['https://api.testnet.shimmer.network'])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
```

Or you can create a `Wallet` instance and interact with it.

```python
from iota_sdk import Wallet, StrongholdSecretManager, CoinType

# This example creates a new database and account

wallet_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

secret_manager = StrongholdSecretManager("wallet.stronghold", "some_hopefully_secure_password")

wallet = Wallet('./alice-database', wallet_options, coin_type=CoinType.SHIMMER, secret_manager)

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
account = wallet.store_mnemonic("flame fever pig forward exact dash body idea link scrub tennis minute " +
          "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

account = wallet.create_account('Alice')
print(account)

```

## Generate API References

You can generate the python API reference with the following command from this directory:

```bash
pip install pydoc-markdown && pydoc-markdown
```