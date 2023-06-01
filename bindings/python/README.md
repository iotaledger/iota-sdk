# IOTA SDK Library - Python binding

Python binding to the [iota-sdk library](/README.md).

## Getting Started

### Requirements

* [Python 3.x](https://www.python.org) 
* [pip ^21.x](https://pypi.org/project/pip)
* `Rust` and `Cargo`, to compile the binding. Install them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Install the iota-sdk

1. Move to the python bindings directory:
    
    ```bash
    cd iota-sdk/bindings/python
    ```

2. (optional) Create a virtual environment and use it. On Linux and macOS you can run the following commands:

    '''bash
    python3 -m venv iota_sdk_venv
    source iota_sdk_venv/bin/activate
    '''

    If you are using Windows you should run the following instead:
    
    ```bash
    .\iota_sdk_venv\Scripts\activate`
    ```

3. Install required dependencies and build the wheel by running the following commands:

    ```bash
    pip install -r requirements-dev.txt
    pip install .
    ````

4. (optional) If you want to deactivate the virtual environment, run the following command:

    ```bash
    deactivate
   ```

## Usage

### Wallet

After you [installed the library](#install-the-iota-sdk), you can create a `Wallet` instance and interact with it.

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

### Client

After you [installed the library](#install-the-iota-sdk), you can create a `Client` instance and interface with it.

```python
from iota_sdk import Client

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
```

### Examples

You can use the provided code [examples](examples) to acquainted with the iota-sdk. You can use the following command to run any example:  

```bash
python3 example/[example file]
```
* Where `[example file]` is the file name from the example folder. For example: 

```bash
python3 examples/client/00_get_info.py
```

### API Reference

You can generate the python API reference with the following command from this directory:

```bash
pipx install pydoc-markdown && pydoc-markdown
```