---
sidebar_label: _node_core_api
title: iota_sdk.client._node_core_api
---

## NodeCoreAPI Objects

```python
class NodeCoreAPI()
```

### get\_node\_health

```python
def get_node_health(url: str)
```

Get node health.

### get\_node\_info

```python
def get_node_info(url: str, auth=None)
```

Get node info.

### get\_info

```python
def get_info()
```

Returns the node information together with the url of the used node.

### get\_peers

```python
def get_peers()
```

Get peers.

### get\_tips

```python
def get_tips() -> List[HexStr]
```

Get tips.

### post\_block

```python
def post_block(block)
```

Post block.

### get\_block\_data

```python
def get_block_data(block_id: HexStr)
```

Post block.

### get\_block\_metadata

```python
def get_block_metadata(block_id: HexStr)
```

Get block metadata with block_id.

### get\_block\_raw

```python
def get_block_raw(block_id: HexStr)
```

Get block raw.

### post\_block\_raw

```python
def post_block_raw(block_bytes)
```

Post block raw.

### get\_output

```python
def get_output(output_id: OutputId)
```

Get output.

### get\_output\_metadata

```python
def get_output_metadata(output_id: OutputId)
```

Get output metadata.

### get\_milestone\_by\_id

```python
def get_milestone_by_id(milestone_id: HexStr)
```

Get the milestone by the given milestone id.

### get\_milestone\_by\_id\_raw

```python
def get_milestone_by_id_raw(milestone_id: HexStr)
```

Get the raw milestone by the given milestone id.

### get\_milestone\_by\_index

```python
def get_milestone_by_index(index: int)
```

Get the milestone by the given index.

### get\_milestone\_by\_index\_raw

```python
def get_milestone_by_index_raw(index: int)
```

Get the milestone by the given index.

### get\_utxo\_changes\_by\_id

```python
def get_utxo_changes_by_id(milestone_id: HexStr)
```

Get the UTXO changes by the given milestone id.

### get\_utxo\_changes\_by\_index

```python
def get_utxo_changes_by_index(index: int)
```

Get the UTXO changes by the given milestone index.

### get\_receipts

```python
def get_receipts()
```

Get all receipts.

### get\_receipts\_migrated\_at

```python
def get_receipts_migrated_at(milestone_index: int)
```

Get the receipts by the given milestone index.

### get\_treasury

```python
def get_treasury()
```

Get the treasury output.

### get\_included\_block

```python
def get_included_block(transaction_id: HexStr)
```

Returns the included block of the transaction.

### get\_included\_block\_metadata

```python
def get_included_block_metadata(transaction_id: HexStr)
```

Returns the metadata of the included block of the transaction.

