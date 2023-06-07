// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will verify an Ed25519 signature.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example verify_ed25519_signature
//! ```

use iota_sdk::{crypto::signatures::ed25519, wallet::Result};

const FOUNDRY_METADATA: &str = r#"{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}"#;
const PUBLIC_KEY: &str = "0x67b7fc3f78763c9394fc4fcdb52cf3a973b6e064bdc3defb40a6cb2c880e6f5c";
const ED25519_SIGNATURE: &str = "0x5437ee671f182507103c6ae2f6649083475019f2cc372e674be164577dd123edd7a76291ba88732bbe1fae39688b50a3678bce05c9ef32c9494b3968f4f07a01";

fn main() -> Result<()> {
    let ed25519_public_key = ed25519::PublicKey::try_from_bytes(prefix_hex::decode(PUBLIC_KEY).unwrap())?;
    let ed25519_signature = ed25519::Signature::from_bytes(prefix_hex::decode(ED25519_SIGNATURE).unwrap());

    let message = FOUNDRY_METADATA.as_bytes();

    let valid_signature = ed25519_public_key.verify(&ed25519_signature, message);
    println!("Valid signature: {valid_signature}");

    Ok(())
}
