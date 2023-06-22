---
title: Get Node Information
description: 'You can access all the features of the iota.rs library using an instance of the Client class. The Client class provides high-level abstraction to all interactions over IOTA network (Tangle).'
image: /img/logo/iota_mark_light.png
keywords:
- how to
- client class
- iota node
- ClientBuilder
- load balancer
- java
- nodejs
- python
- rust
---

import CodeBlock from '@theme/CodeBlock';
import Tabs from "@theme/Tabs";
import TabItem from "@theme/TabItem";
import NodejsCode from '!!raw-loader!../../../../../bindings/nodejs/examples/how_tos/client/get-info.ts';
import PythonCode from '!!raw-loader!../../../../../bindings/python/examples/how_tos/client/get_info.py';
import RustCode from '!!raw-loader!../../../../../sdk/examples/how_tos/client/get_info.rs';
import AccountClient from '../../_admonitions/_account-client.md'

Sometimes it's needed to get certain info from the Node to determine for example if the node is synced or which features it has enabled. You can get this info from a Client instance.

<AccountClient/>

The following code example will:

1. Create a `Client` which will connect to the [Shimmer Testnet](https://api.testnet.shimmer.network).
2. Use the created client to get information about the node.
3. Print the information to the console.

## Code Example

<Tabs groupId="language">
    <TabItem value="rust" label="Rust">
        <CodeBlock className="language-rust">
            {RustCode}
        </CodeBlock>
    </TabItem>
    <TabItem value="nodejs" label="Nodejs">
        <CodeBlock className="language-typescript">
            {NodejsCode}
        </CodeBlock>
    </TabItem>
    <TabItem value="python" label="Python">
        <CodeBlock className="language-python">
            {PythonCode}
        </CodeBlock>
    </TabItem>
</Tabs>

## Expected Output

<Tabs groupId="language">
<TabItem value="rust" label="Rust">

```bash
InfoResponse {
    name: "HORNET",
    version: "2.0.0-rc.6",
    status: StatusResponse {
        is_healthy: true,
        latest_milestone: LatestMilestoneResponse {
            index: 5792633,
            timestamp: Some(
                1687456380,
            ),
            milestone_id: Some(
                "0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc",
            ),
        },
        confirmed_milestone: ConfirmedMilestoneResponse {
            index: 5792633,
            timestamp: Some(
                1687456380,
            ),
            milestone_id: Some(
                "0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc",
            ),
        },
        pruning_index: 4750998,
    },
    supported_protocol_versions: [
        2,
    ],
    protocol: ProtocolParametersDto {
        protocol_version: 2,
        network_name: "testnet-1",
        bech32_hrp: Hrp {
            inner: [
                114,
                109,
                115,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            len: 3,
        },
        min_pow_score: 1500,
        below_max_depth: 15,
        rent_structure: RentStructure {
            v_byte_cost: 100,
            v_byte_factor_key: 10,
            v_byte_factor_data: 1,
        },
        token_supply: "1450896407249092",
    },
    pending_protocol_parameters: [],
    base_token: BaseTokenResponse {
        name: "Shimmer",
        ticker_symbol: "SMR",
        unit: "SMR",
        subunit: Some(
            "glow",
        ),
        decimals: 6,
        use_metric_prefix: false,
    },
    metrics: MetricsResponse {
        blocks_per_second: 1.4,
        referenced_blocks_per_second: 0.2,
        referenced_rate: 14.285714285714285,
    },
    features: [],
}
```

</TabItem>
<TabItem value="nodejs" label="Nodejs">

````bash
Node info:  {
  name: 'HORNET',
  version: '2.0.0-rc.6',
  status: {
    isHealthy: true,
    latestMilestone: {
      index: 5792633,
      timestamp: 1687456380,
      milestoneId: '0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc'
    },
    confirmedMilestone: {
      index: 5792633,
      timestamp: 1687456380,
      milestoneId: '0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc'
    },
    pruningIndex: 4750998
  },
  supportedProtocolVersions: [ 2 ],
  protocol: {
    version: 2,
    networkName: 'testnet-1',
    bech32Hrp: 'rms',
    minPowScore: 1500,
    belowMaxDepth: 15,
    rentStructure: { vByteCost: 100, vByteFactorKey: 10, vByteFactorData: 1 },
    tokenSupply: '1450896407249092'
  },
  pendingProtocolParameters: [],
  baseToken: {
    name: 'Shimmer',
    tickerSymbol: 'SMR',
    unit: 'SMR',
    subunit: 'glow',
    decimals: 6,
    useMetricPrefix: false
  },
  metrics: {
    blocksPerSecond: 1.4,
    referencedBlocksPerSecond: 0.2,
    referencedRate: 14.285714285714285
  },
  features: []
}
````

</TabItem>
<TabItem value="python" label="Python">

```bash
{
    "name": "HORNET",
    "version": "2.0.0-rc.6",
    "status": {
        "isHealthy": true,
        "latestMilestone": {
            "index": 5792633,
            "timestamp": 1687456380,
            "milestoneId": "0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc"
        },
        "confirmedMilestone": {
            "index": 5792633,
            "timestamp": 1687456380,
            "milestoneId": "0x5d554e0c20779dae25288efefb33c385b11c2dc6088f9418d3a1fececa1385fc"
        },
        "pruningIndex": 4750998
    },
    "supportedProtocolVersions": [
        2
    ],
    "protocol": {
        "version": 2,
        "networkName": "testnet-1",
        "bech32Hrp": "rms",
        "minPowScore": 1500,
        "belowMaxDepth": 15,
        "rentStructure": {
            "vByteCost": 100,
            "vByteFactorKey": 10,
            "vByteFactorData": 1
        },
        "tokenSupply": "1450896407249092"
    },
    "pendingProtocolParameters": [],
    "baseToken": {
        "name": "Shimmer",
        "tickerSymbol": "SMR",
        "unit": "SMR",
        "subunit": "glow",
        "decimals": 6,
        "useMetricPrefix": false
    },
    "metrics": {
        "blocksPerSecond": 1.4,
        "referencedBlocksPerSecond": 0.2,
        "referencedRate": 14.285714285714285
    },
    "features": []
}
```

</TabItem>
</Tabs>
