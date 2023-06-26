---
title: Get Node Health
description: 'Check any nodes health.'
image: /img/logo/iota_mark_light.png
keywords:
- how to
- client
- load balancer
- node health
- nodejs
- python
- rust
---

import CodeBlock from '@theme/CodeBlock';
import Tabs from "@theme/Tabs";
import TabItem from "@theme/TabItem";
import NodejsCode from '!!raw-loader!../../../../../bindings/nodejs/examples/how_tos/client/get-health.ts';
import PythonCode from '!!raw-loader!../../../../../bindings/python/examples/how_tos/client/get_health.py';
import RustCode from '!!raw-loader!../../../../../sdk/examples/how_tos/client/get_health.rs';
import AccountClient from '../../_admonitions/_account-client.md'

You can check the health of any node.

<AccountClient/>

The following code example will:

1. Create a `Client` which will connect to the [Shimmer Testnet](https://api.testnet.shimmer.network).
2. Use the created client to get the health of the specified url.
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

```bash
Healthy: true
```
