# IOTA SDK Library - Python binding

Python binding to the [iota-sdk library](/README.md).

## Table of contents

- [IOTA SDK Library - Python binding](#iota-sdk-library---python-binding)
  - [Table of contents](#table-of-contents)
  - [Requirements](#requirements)
  - [Getting Started](#getting-started)
    - [Install the IOTA SDK via pip](#install-iota-sdk-via-pip)
    - [Install the IOTA SDK](#install-the-iota-sdk)
  - [Client](#client-usage)
  - [Wallet](#wallet-usage)
  - [Examples](#examples)
  - [API Reference](#api-reference)
  - [Learn More](#learn-more)

## Requirements

- [Python 3.10+](https://www.python.org)
- [pip ^21.x](https://pypi.org/project/pip)
- `Rust` and `Cargo` to compile the binding. Install
  them [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Getting Started

### Install IOTA SDK via pip

1. (optional) Create a virtual environment and use it. On Linux and macOS, you can run the following commands:

   ```bash
   python3 -m venv env
   source env/bin/activate
   ```

   If you are using Windows, you should run the following instead:

   ```bash
   python3 -m venv env
   .\env\Scripts\activate
   ```

2. Install the IOTA-SDK using pip:

   ```bash
   pip install iota-sdk
   ```

3. (optional) If you want to deactivate the virtual environment, run the following command:

   ```bash
   deactivate
   ```

### Install the IOTA SDK

1. Move to the Python bindings directory:

   ```bash
   cd iota-sdk/bindings/python
   ```

2. (optional) Create a virtual environment and use it. On Linux and macOS, you can run the following commands:

   ```bash
   python3 -m venv env
   source env/bin/activate
   ```

   If you are using Windows, you should run the following instead:

   ```bash
   python3 -m venv env
   .\env\Scripts\activate
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

The following example creates a Client instance connected to the Shimmer Testnet, and retrieves the node's information by calling `Client.get_info()`, and then print the node's information.

[examples/client/getting_started.py](examples/client/getting_started.py)

## Wallet Usage

The following example will create a new Wallet using a StrongholdSecretManager, and then print the wallet's information.

[examples/wallet/getting_started.py](examples/wallet/getting_started.py)

## Examples

You can use the provided code [examples](https://github.com/iotaledger/iota-sdk/blob/develop/bindings/python/examples) to acquainted with the IOTA SDK. You can use the following command to
run any example:

```bash
python3 example/[example file]
```

- Where `[example file]` is the file name from the example folder. For example:

```bash
python3 examples/how_tos/client/get_info.py
```

## API Reference

You can find the API reference for the Python bindings in the
[IOTA Wiki](https://wiki.iota.org/shimmer/iota-sdk/references/python/iota_sdk/client/).


## Learn More

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).
