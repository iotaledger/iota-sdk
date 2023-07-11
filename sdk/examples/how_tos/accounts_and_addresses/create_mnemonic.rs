// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a random mnemonic.
//!
//! ```sh
//! `cargo run --release --all-features --example create_mnemonic`
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;

    println!("Generated mnemonic:\n{}", mnemonic.as_ref());

    Ok(())
}
