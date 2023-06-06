---
description: Get started with the IOTA Wallet Library and choose your programming language.
image: /img/logo/wallet_light.png
keywords:

- rust
- node.js
- python
- wasm
- javascript
- wallet
- accounts

---

# Getting Started

## About the IOTA SDK

The IOTA SDK is a Rust-based project that provides a convenient and efficient way to interact with nodes in the
Shimmer and IOTA networks running
the [Stardust protocol](https://wiki.iota.org/shimmer/develop/explanations/what-is-stardust). It consists of two main
modules: `client` and `wallet`.

The `client` module is stateless. It aims to provide more flexibility and access to low-level functions.

The `wallet` module is stateful, with a standardized interface for developers to build applications involving value
transactions. It uses high-level functions that simplify everyday operations. It can optionally interact
with [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs/) for seed handling, storage, and state backup.

You can use this section to get your wallet up and running in
the [programming language of your choice](#available-programming-languages),
[connect to the testnet](#connect-to-the-testnet-api), [explore the network](#explore-the-network),
and [get test tokens](#get-test-tokens) to develop your application.

## Connect to the Testnet API

We recommended that you start your interactions with Shimmer on a _testnet_ network. The _testnet_ will allow you to
safely get acquainted with the `wallet.rs` library, without the risk of losing any funds if you make a mistake along the
way.

You can use this public load-balanced Shimmer Testnet API:

```plaintext
https://api.testnet.shimmer.network
```

## Explore the Network

You can use the [Shimmer Tangle Explorer](https://explorer.shimmer.network/testnet) to view transactions and data stored
in the Tangle.

## Get Test Tokens

In order to properly test value-based transactions on testnet network, you are going to need some tokens. You can get
some testnet tokens through the [Shimmer Faucet](https://faucet.testnet.shimmer.network).

## Available Programming Languages

The wallet.rs library is written in [Rust](./rust.mdx), and also has convenient bindings
in [Node.js](./nodejs.mdx), [Python](./python.mdx) and [Wasm](./wasm.mdx). Each of these languages has specific
instructions you will need to follow to use wallet.rs in your project. 