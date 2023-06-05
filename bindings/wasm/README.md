# IOTA SDK Library - WebAssembly Bindings

WebAssembly (Wasm) bindings for TypeScript/JavaScript to the IOTA SDK library.

## Which Bindings to Choose?

The IOTA SDK library also offers dedicated [Node.js bindings](../nodejs). The differences with this package are outlined
below.

|              |   Wasm bindings   |   Node.js bindings    |
|:-------------|:-----------------:|:---------------------:|
| Environment  | Node.js, browsers |        Node.js        |
| Installation |         -         | Rust, Cargo required* |
| Performance  |        ✔️         |         ✔️✔️          |
| Ledger Nano  |         ❌         |          ✔️           |
| Rocksdb      |         ❌         |          ✔️           |
| Stronghold   |         ❌         |          ✔️           |

*Node.js bindings only need to be compiled during `npm install` if a pre-compiled binary is not available for your
platform.

**tl;dr: Use the Node.js bindings if you can. The Wasm bindings are just more portable and support browser environments.
**

## Requirements

- One of the following Node.js versions: '16.x', '18.x';
- `wasm-bindgen` (`cargo install wasm-bindgen-cli`);

## Getting Started

### Installing the IOTA SDK

#### Install With a Package Manager

To install the library from your package manager of choice, you only need to run the following:

##### npm

```bash
$ npm i @iota/sdk-wasm
```

##### yarn

```bash
$ yarn add @iota/sdk-wasm
```

### Usage

##### Node.js

##### Client

After installing the library, you can create a `Client` instance and interface with it.

```javascript
const {Client, initLogger} = require('@iota/sdk-wasm/node');

async function run() {
    initLogger();

    const client = new Client({
        nodes: ['https://api.testnet.shimmer.network'],
        localPow: true,
    });

    try {
        const nodeInfo = await client.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
```

##### Wallet

After you [installed the library](#installing-the-iota-sdk), you can create a `Wallet` instance and interact with it.

```javascript
const {Wallet, CoinType} = require('@iota/sdk-wasm/node');

const wallet = new Wallet({
    storagePath: './my-database',
    coinType: CoinType.Shimmer,
    clientOptions: {
        nodes: ['https://api.testnet.shimmer.network'],
    },
    secretManager: {
        mnemonic: "my development mnemonic",
    },
});

const account = await wallet.createAccount({
    alias: 'Alice',
});

account.addresses().then((addresses) => {
    console.log(addresses);
});
```

See the [Node.js examples](../nodejs/examples) for more demonstrations; the only change needed is to import
`@iota/sdk-wasm/node` instead of `@iota/sdk`.

### Web Setup

Unlike Node.js, a few more steps are required to use this binding in a browser.

The library loads the compiled Wasm file with an HTTP GET request, so the `iota_sdk_wasm_bg.wasm` file must be copied to
the root of the distribution folder.

A bundler such as [webpack](https://webpack.js.org/) or [rollup](https://rollupjs.org/) is recommended.

#### Rollup

1. Install `rollup-plugin-copy`:

    ```bash
    npm install rollup-plugin-copy --save-dev
    ```

2. Add the plugin to your `rollup.config.js`:

    ```js
    // Include the copy plugin.
    import copy from 'rollup-plugin-copy'
    
    // ...
    
    // Add the copy plugin to the `plugins` array:
    copy({
      targets: [{
        src: 'node_modules/@iota/sdk-wasm/web/wasm/iota_sdk_wasm_bg.wasm',
        dest: 'public',
        rename: 'iota_sdk_wasm_bg.wasm'
      }]
    })
    ```

#### Webpack

1. Install `copy-webpack-plugin`:

    ```bash
    npm install copy-webpack-plugin --save-dev
    ```

2. Add the plugin to your `webpack.config.js`:

    ```js
    // Include the copy plugin.
    const CopyWebPlugin = require('copy-webpack-plugin');
    
    // ...
    
    experiments: {
        // futureDefaults: true, // includes asyncWebAssembly, topLevelAwait etc.
        asyncWebAssembly: true
    }
    
    // Add the copy plugin to the `plugins` array:
    plugins: [
        new CopyWebPlugin({
          patterns: [
            {
              from: 'node_modules/@iota/sdk-wasm/web/wasm/iota_sdk_wasm_bg.wasm',
              to: 'iota_sdk_wasm_bg.wasm'
            }
          ]
        }),
        // other plugins...
    ]
    ```

### Web Usage

```javascript
import init, {Wallet, CoinType} from "@iota/sdk-wasm/web";

init().then(() => {
    const wallet = new Wallet({
        storagePath: './my-database',
        coinType: CoinType.Shimmer,
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
        },
        secretManager: {
            mnemonic: "my development mnemonic",
        },
    });

    const account = await wallet.createAccount({
        alias: 'Alice',
    });

    account.addresses().then((addresses) => {
        console.log(addresses);
    });
}).catch(console.error);

// Default path to load is "iota_sdk_wasm_bg.wasm", 
// but you can override it by passing a path explicitly.
//
// init("./static/iota_sdk_wasm_bg.wasm").then(...)
```
