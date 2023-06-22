---
sidebar_label: feature
title: iota_sdk.types.feature
---

## Feature Objects

```python
class Feature()
```

### \_\_init\_\_

```python
def __init__(type, sender=None, issuer=None, data=None, tag=None)
```

Initialize a feature

#### Parameters

* __type__*: FeatureType*  
    The type of feature
* __sender__*: Address*  
    Sender address
* __issuer__*: Address*  
    Issuer Address
* __data__*: string*  
    Hex encoded metadata
* __tag__*: string*  
    Hex encoded tag used to index the output

## SenderFeature Objects

```python
class SenderFeature(Feature)
```

### \_\_init\_\_

```python
def __init__(sender)
```

Initialize a SenderFeature

#### Parameters

* __sender__*: Address*  
    Sender address

## IssuerFeature Objects

```python
class IssuerFeature(Feature)
```

### \_\_init\_\_

```python
def __init__(issuer)
```

Initialize an IssuerFeature

#### Parameters

* __issuer__*: Address*  
    Issuer address

## MetadataFeature Objects

```python
class MetadataFeature(Feature)
```

### \_\_init\_\_

```python
def __init__(data: HexStr)
```

Initialize a MetadataFeature

#### Parameters

* __data__*: HexStr*  
    Hex encoded metadata

## TagFeature Objects

```python
class TagFeature(Feature)
```

### \_\_init\_\_

```python
def __init__(tag: HexStr)
```

Initialize a TagFeature

#### Parameters

* __tag__*: HexStr*  
    Hex encoded tag used to index the output

