// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{Client, ClientBuilder},
    types::block::protocol::ProtocolParameters,
};

#[tokio::test]
async fn invalid_url() {
    let client = Client::builder().with_node("data:text/plain,Hello?World#");
    assert!(client.is_err());
}

#[tokio::test]
async fn valid_url() {
    let client = Client::builder().with_node("http://localhost:8050");
    assert!(client.is_ok());
}

#[tokio::test]
async fn client_builder() {
    let client_builder_json = serde_json::json!({
        "nodes": [
            {
                "url":"http://localhost:8050/",
                "disabled": false
            }
        ],
        "ignoreNodeHealth": true,
        "nodeSyncInterval": {
            "secs": 60,
            "nanos": 0
        },
        "quorum": false,
        "minQuorumSize": 3,
        "quorumThreshold": 66,
        "userAgent": "iota-client/2.0.1-rc.3",
        "protocolParameters": ProtocolParameters::default(),
        "apiTimeout": {
            "secs": 15,
            "nanos": 0
        }
    });

    let _client_builder = serde_json::from_value::<ClientBuilder>(client_builder_json).unwrap();
}
