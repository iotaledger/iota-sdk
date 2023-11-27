// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This examples shows how to create log files.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example client_logger
//! ```

use iota_sdk::client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    #[allow(clippy::single_element_loop)]
    for var in ["NODE_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Generates a client.log file with logs for debugging.
    // We exclude logs from h2, hyper and rustls to reduce the noise.
    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name("client.log")
        .target_exclusions(&["h2", "hyper", "rustls"])
        .level_filter(log::LevelFilter::Debug);

    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();

    fern_logger::logger_init(config).unwrap();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    // Get node info.
    let _ = client.get_info().await?;

    println!("Example completed successfully. `client.log` file has been updated.");

    Ok(())
}
