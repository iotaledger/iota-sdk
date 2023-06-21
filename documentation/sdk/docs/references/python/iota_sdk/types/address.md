---
sidebar_label: address
title: iota_sdk.types.address
---

## Address Objects

```python
class Address()
```

### \_\_init\_\_

```python
def __init__(type: AddressType, address_or_id: HexStr)
```

Initialize an Address

#### Parameters

* __type__*: AddressType*  
    The type of the Address
* __address_or_id__*: string*  
    The address to use. Can either be an hex encoded ED25519 address or NFT/Alias id

## Ed25519Address Objects

```python
class Ed25519Address(Address)
```

### \_\_init\_\_

```python
def __init__(address: HexStr)
```

Initialize an Ed25519Address

#### Parameters

* __address__*: string*  
    The hex encoded address to use.

## AliasAddress Objects

```python
class AliasAddress(Address)
```

### \_\_init\_\_

```python
def __init__(address_or_id: HexStr)
```

Initialize an AliasAddress

#### Parameters

* __address_or_id__*: string*  
    The hex encoded address to use.

## NFTAddress Objects

```python
class NFTAddress(Address)
```

### \_\_init\_\_

```python
def __init__(address_or_id: HexStr)
```

Initialize an NFTokenAddress

#### Parameters

* __address_or_id__*: string*  
    The hex encoded address to use.

