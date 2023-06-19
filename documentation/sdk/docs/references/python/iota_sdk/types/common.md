---
sidebar_label: common
title: iota_sdk.types.common
---

## Node Objects

```python
class Node()
```

### \_\_init\_\_

```python
def __init__(url=None, jwt=None, username=None, password=None, disabled=None)
```

Initialize a Node

#### Parameters

* __url__*: string*  
    Node url
* __jwt__*: string*  
    JWT token
* __username__*: string*  
    Username for basic authentication
* __password__*: string*  
    Password for basic authentication
* __disabled__*: bool*  
    Disable node

## SendAmountParams Objects

```python
class SendAmountParams()
```

### \_\_init\_\_

```python
def __init__(address, amount)
```

Initialise SendAmountParams

#### Parameters

* __address__*: string*  
    Address of the output
* __amount__*: int*  
    Amount of the output

