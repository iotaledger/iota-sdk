---
sidebar_label: client
title: iota_sdk.client.client
---

## ClientError Objects

```python
class ClientError(Exception)
```

client error

## Client Objects

```python
class Client(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, ClientUtils)
```

### \_\_init\_\_

```python
def __init__(nodes: Optional[str | List[str]] = None,
             primary_node: Optional[str] = None,
             primary_pow_node: Optional[str] = None,
             permanode: Optional[str] = None,
             ignore_node_health: Optional[bool] = None,
             api_timeout: Optional[timedelta] = None,
             node_sync_interval: Optional[timedelta] = None,
             remote_pow_timeout: Optional[timedelta] = None,
             tips_interval: Optional[int] = None,
             quorum: Optional[bool] = None,
             min_quorum_size: Optional[int] = None,
             quorum_threshold: Optional[int] = None,
             user_agent: Optional[str] = None,
             local_pow: Optional[bool] = None,
             fallback_to_local_pow: Optional[bool] = None,
             pow_worker_count: Optional[int] = None,
             client_handle=None)
```

Initialize the IOTA Client.

#### Parameters

* __nodes__*: string or array of string*  
    A single Node URL or an array of URLs.
* __primary_node__*: string*  
    Node which will be tried first for all requests.
* __primary_pow_node__*: string*  
    Node which will be tried first when using remote PoW, even before the primary_node.
* __permanode__*: string*  
    Permanode URL.
* __ignore_node_health__*: bool*  
    If the node health should be ignored.
* __api_timeout__*: datetime.timedelta*  
    Timeout for API requests.
* __node_sync_interval__*: datetime.timedelta*  
    Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
* __remote_pow_timeout__*: datetime.timedelta*  
    Timeout when sending a block that requires remote proof of work.
* __tips_interval__*: int*  
    Tips request interval during PoW in seconds.
* __quorum__*: bool*  
    If node quorum is enabled. Will compare the responses from multiple nodes 
    and only returns the response if `quorum_threshold`% of the nodes return the same one.
* __min_quorum_size__*: int*  
    Minimum amount of nodes required for request when quorum is enabled.
* __quorum_threshold__*: int*  
    % of nodes that have to return the same response so it gets accepted.
* __user_agent__*: string*  
    The User-Agent header for requests.
* __local_pow__*: bool*  
    Local proof of work.
* __fallback_to_local_pow__*: bool*  
    Fallback to local proof of work if the node doesn&#x27;t support remote PoW.
* __pow_worker_count__*: int*  
    The amount of threads to be used for proof of work.

### build\_alias\_output

```python
def build_alias_output(alias_id: HexStr,
                       unlock_conditions: List[UnlockCondition],
                       amount: Optional[int] = None,
                       native_tokens: Optional[List[NativeToken]] = None,
                       state_index: Optional[int] = None,
                       state_metadata: Optional[str] = None,
                       foundry_counter: Optional[int] = None,
                       features: Optional[List[Feature]] = None,
                       immutable_features: Optional[List[Feature]] = None)
```

Build an AliasOutput.

#### Parameters

* __alias_id__*: string*  
    Hex encoded alias id
* __unlock_conditions__*: array of UnlockCondition*  
    The unlock conditions for this output
* __amount__*: int*  
    Amount of base token
* __native_tokens__*: array of NativeToken*  
    The native token to add to the output
* __state_index__*: int*  
    Index of the state
* __state_metadata__*: string*  
    Hex encoded state metadata
* __foundry_counter__*: int*  
    Counter of foundry outputs created
* __features__*: array of Feature*  
    Features for this outputs
* __immutable_features__*: array of Feature*  
    Immutable features

#### Returns

Output as dict

### build\_basic\_output

```python
def build_basic_output(unlock_conditions: List[UnlockCondition],
                       amount: Optional[int] = None,
                       native_tokens: Optional[List[NativeToken]] = None,
                       features: Optional[List[Feature]] = None)
```

Build a BasicOutput.

#### Parameters

* __unlock_conditions__*: array of UnlockCondition*  
    The unlock conditions for this output
* __amount__*: int*  
    Amount of base token
* __native_tokens__*: array of NativeToken*  
    The native token to add to the output
* __features__*: array of Feature*  
    Features for this outputs

#### Returns

Output as dict

### build\_foundry\_output

```python
def build_foundry_output(serial_number: int,
                         token_scheme: TokenScheme,
                         unlock_conditions: List[UnlockCondition],
                         amount: Optional[int] = None,
                         native_tokens: Optional[List[NativeToken]] = None,
                         features: Optional[List[Feature]] = None,
                         immutable_features: Optional[List[Feature]] = None)
```

Build a FoundryOutput.

#### Parameters

* __serial_number__*: int*  
    The serial number of the foundry
* __token_scheme__*: TokenScheme*  
    The Token scheme. Currently only a simple scheme is supported
* __unlock_conditions__*: array of UnlockCondition*  
    The unlock conditions for this output
* __amount__*: int*  
    Amount of base token
* __native_tokens__*: array of NativeToken*  
    The native token to add to the output
* __features__*: array of Feature*  
    Features for this outputs
* __immutable_features__*: array of Feature*  
    Immutable features

#### Returns

Output as dict

### build\_nft\_output

```python
def build_nft_output(nft_id: HexStr,
                     unlock_conditions: List[UnlockCondition],
                     amount: Optional[int] = None,
                     native_tokens: Optional[List[NativeToken]] = None,
                     features: Optional[List[Feature]] = None,
                     immutable_features: Optional[List[Feature]] = None)
```

Build an NftOutput.

#### Parameters

* __nft_id__*: string*  
    Hex encoded nft id
* __unlock_conditions__*: array of UnlockCondition*  
    The unlock conditions for this output
* __amount__*: int*  
    Amount of base token
* __native_tokens__*: array of NativeToken*  
    The native tokens to add to the output
* __features__*: array of Feature*  
    Features for this outputs
* __immutable_features__*: array of Feature*  
    Immutable features

#### Returns

Output as dict

### build\_and\_post\_block

```python
def build_and_post_block(secret_manager=None,
                         account_index: Optional[int] = None,
                         coin_type: Optional[int] = None,
                         custom_remainder_address: Optional[str] = None,
                         data: Optional[HexStr] = None,
                         initial_address_index: Optional[int] = None,
                         input_range_start: Optional[int] = None,
                         input_range_end: Optional[int] = None,
                         inputs: Optional[List[Dict[str, Any]]] = None,
                         output: Optional[Dict[str, Any]] = None,
                         outputs: Optional[List[Any]] = None,
                         tag: Optional[HexStr] = None)
```

Build and post a block.

#### Parameters

* __account_index__*: int*  
    Account Index
* __coin_type__*: int*  
    Coin type. The CoinType enum can be used
* __custom_remainder_address__*: string*  
    Address to send the remainder funds to
* __data__*: str*  
    Hex encoded data
* __initial_address_index__*: int*  
    Initial address index
* __input_range_start__*: int*  
    Start of the input range
* __input_range_end__*: int*  
    End of the input range
* __inputs__*: Array of Inputs*  
    Inputs to use
* __output__*: SendAmountParams*  
    Address and amount to send to
* __outputs__*: Array of Outputs*  
    Outputs to use
* __tag__*: string*  
    Hex encoded tag

#### Returns

Block as dict

### get\_node

```python
def get_node() -> Dict[str, Any]
```

Get a node candidate from the healthy node pool.

### get\_network\_info

```python
def get_network_info() -> Dict[str, Any]
```

Gets the network related information such as network_id and min_pow_score.

### get\_network\_id

```python
def get_network_id() -> int
```

Gets the network id of the node we&#x27;re connecting to.

### get\_bech32\_hrp

```python
def get_bech32_hrp() -> str
```

Returns the bech32_hrp.

### get\_min\_pow\_score

```python
def get_min_pow_score() -> int
```

Returns the min pow score.

### get\_tips\_interval

```python
def get_tips_interval() -> int
```

Returns the tips interval.

### get\_local\_pow

```python
def get_local_pow() -> bool
```

Returns if local pow should be used or not.

### get\_fallback\_to\_local\_pow

```python
def get_fallback_to_local_pow() -> bool
```

Get fallback to local proof of work timeout.

### unhealthy\_nodes

```python
def unhealthy_nodes() -> List[Dict[str, Any]]
```

Returns the unhealthy nodes.

### prepare\_transaction

```python
def prepare_transaction(secret_manager=None, options=None)
```

Prepare a transaction for signing.

### sign\_transaction

```python
def sign_transaction(secret_manager, prepared_transaction_data)
```

Sign a transaction.

### submit\_payload

```python
def submit_payload(payload)
```

Submit a payload in a block.

