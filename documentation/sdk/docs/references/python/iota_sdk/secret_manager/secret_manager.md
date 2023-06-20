---
sidebar_label: secret_manager
title: iota_sdk.secret_manager.secret_manager
---

## LedgerNanoSecretManager Objects

```python
class LedgerNanoSecretManager(dict)
```

Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator.

### \_\_init\_\_

```python
def __init__(is_simulator)
```

Initialize a ledger nano secret manager.

## MnemonicSecretManager Objects

```python
class MnemonicSecretManager(dict)
```

Secret manager that uses a mnemonic in plain memory. It&#x27;s not recommended for production use. Use LedgerNano or Stronghold instead.

### \_\_init\_\_

```python
def __init__(mnemonic)
```

Initialize a mnemonic secret manager.

## SeedSecretManager Objects

```python
class SeedSecretManager(dict)
```

### \_\_init\_\_

```python
def __init__(seed)
```

Initialize a seed secret manager.

## StrongholdSecretManager Objects

```python
class StrongholdSecretManager(dict)
```

Secret manager that uses Stronghold.

### \_\_init\_\_

```python
def __init__(snapshot_path, password)
```

Initialize a stronghold secret manager.

## SecretManagerError Objects

```python
class SecretManagerError(Exception)
```

secret manager error

## SecretManager Objects

```python
class SecretManager()
```

### generate\_ed25519\_addresses

```python
def generate_ed25519_addresses(account_index: Optional[int] = None,
                               start: Optional[int] = None,
                               end: Optional[int] = None,
                               internal: Optional[bool] = None,
                               coin_type: Optional[int] = None,
                               bech32_hrp: Optional[str] = None,
                               ledger_nano_prompt: Optional[bool] = None)
```

Generate ed25519 addresses.

#### Parameters

* __account_index__*: int*  
    Account index.
* __start__*: int*  
    Start index of generated addresses
* __end__*: int*  
    End index of generated addresses
* __internal__*: bool*  
    Internal addresses
* __coin_type__*: int*  
    Coin type. The CoinType enum can be used
* __bech32_hrp__*: string*  
    Bech32 human readable part.
* __ledger_nano_prompt__*: bool*  
    Display the address on ledger devices.

#### Returns

Addresses as array of strings.

### generate\_evm\_addresses

```python
def generate_evm_addresses(account_index=None,
                           start=None,
                           end=None,
                           internal=None,
                           coin_type=None,
                           ledger_nano_prompt=None)
```

Generate EVM addresses.

#### Parameters

* __account_index__*: int*  
    Account index.
* __start__*: int*  
    Start index of generated addresses
* __end__*: int*  
    End index of generated addresses
* __internal__*: bool*  
    Internal addresses
* __coin_type__*: int*  
    Coin type. The CoinType enum can be used
* __ledger_nano_prompt__*: bool*  
    Display the address on ledger devices.

#### Returns

Addresses as array of strings.

### get\_ledger\_nano\_status

```python
def get_ledger_nano_status()
```

Returns the Ledger Status.

### store\_mnemonic

```python
def store_mnemonic(mnemonic: str)
```

Store a mnemonic in the Stronghold vault.

### sign\_ed25519

```python
def sign_ed25519(message: HexStr, chain: List[int])
```

Signs a message with an Ed25519 private key.

### sign\_secp256k1\_ecdsa

```python
def sign_secp256k1_ecdsa(message: HexStr, chain: List[int])
```

Signs a message with an Secp256k1Ecdsa private key.

### sign\_transaction

```python
def sign_transaction(prepared_transaction_data)
```

Sign a transaction.

### signature\_unlock

```python
def signature_unlock(transaction_essence_hash: HexStr, chain: List[int])
```

Sign a transaction essence hash.

