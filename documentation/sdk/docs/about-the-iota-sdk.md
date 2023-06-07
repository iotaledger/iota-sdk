--- 
description: Get started with the IOTA SDK and choose your programming language.
image: /img/logo/wallet_light.png
keywords:
- rust
- node.js
- python
- wasm
- javascript
- wallet
- client
- SDK
- accounts
---  

# About the IOTA SDK

![IOTA SDK Overview](/img/banner/client_lib_overview.png)

The IOTA SDK is a Rust-based project that provides a convenient and efficient way to interact with nodes in the Shimmer
and IOTA networks running the [Stardust protocol](https://wiki.iota.org/shimmer/develop/explanations/what-is-stardust).
It consists of two main modules: `client` and `wallet`.

The `client` module is stateless. It aims to provide more flexibility and access to low-level functions.

The `wallet` module is stateful, with a standardized interface for developers to build applications involving value
transactions. It uses high-level functions that simplify everyday operations. It can optionally interact
with [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs/) for seed handling, storage, and state backup.

You can use this documentation to get your `wallet` or `client` up and running in
the [programming language of your choice](#available-programming-languages), [connect to the testnet](explanations/testnet-and-test-tokens.md#connect-to-the-testnet-api), [explore the network](explanations/testnet-and-test-tokens.md#explore-the-network),
and [get test tokens](explanations/testnet-and-test-tokens.md#get-test-tokens) to develop your application.

## Available Programming Languages

The IOTA SDK is written in [Rust](getting-started/rust.mdx) and also has convenient bindings
in [Node.js](getting-started/nodejs.mdx), [Python](getting-started/python.mdx) and [Wasm](getting-started/wasm.mdx).

Each of these languages has specific instructions you will need to follow to use IOTA SDK in your project. Every binding
is adjusted for the language's conventions and best practices. For example, Python developers avoid the Builder
programming pattern, so our Python binding uses named constructor arguments. However, we always keep the meaning behind
our API, which is equally powerful no matter which language you choose.

## Your Application In the IOTA Network

Your application communicates directly with the IOTA SDK in Rust or through one of the language bindings. IOTA SDK turns
your requests into REST API calls and sends them to a node through the Internet. The node, in turn, interacts with the
rest of an IOTA network, which could be
the [main operational network (mainnet)](https://wiki.iota.org/shimmer/develop/explanations/what-is-shimmer/networks/#shimmer-mainnet)
or
a [network for testing purposes (testnet)](https://wiki.iota.org/shimmer/develop/explanations/what-is-shimmer/networks/#public-testnet).   !["An overview of IOTA layers."](/img/iota_layers_overview.svg "An overview of IOTA layers.")

## Secure Secret Management

You can use [Stronghold](https://wiki.iota.org/shimmer/stronghold.rs/welcome) for secure secret management. Stronghold
can store the encrypted seed at rest. It is not possible to extract the seed from Stronghold for security purposes.
Stronghold uses encrypted snapshots that can easily be backed up and securely shared between devices. These snapshots
are further secured with a password.

## Join the Discussion

If you want to get involved in discussions about this library, or you're looking for support, go to the TODO channel
on [Discord](https://discord.iota.org). 


