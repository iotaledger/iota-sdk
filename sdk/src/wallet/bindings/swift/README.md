# IOTA Wallet Library - Swift binding

Swift binding to the IOTA wallet library

## Requirements

Ensure you have first installed the latest stable version of Rust and Cargo.

## Installation

For current system architecture
```
cd iota-sdk/bindings/swift
cargo build
```

For debug build, copy `iota-sdk/bindings/swift/iota_wallet_ffi.h` and `iota-sdk/bindings/swift/target/debug/libiota_wallet.dylib` to `iota-sdk/bindings/swift/xcode/IotaWallet/iota_wallet`.

Open and `build iota-sdk/bindings/swift/xcode/IotaWallet/IotaWallet.xcodeproj`. The xcode build product is an Objective-C framework that can be used in Swift.

