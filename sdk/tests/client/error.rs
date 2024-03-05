// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{api::transaction_builder::TransactionBuilderError, ClientError},
    types::block::BlockError,
};
use pretty_assertions::assert_eq;

#[test]
fn stringified_error() {
    let error = ClientError::InvalidAmount("0".into());
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"invalidAmount\",\"error\":\"invalid amount in API response: 0\"}"
    );

    let error = ClientError::TimeNotSynced {
        current_time: 0,
        tangle_time: 10000,
    };
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"timeNotSynced\",\"error\":\"local time 0 doesn't match the tangle time: 10000\"}"
    );

    let error = ClientError::PlaceholderSecretManager;
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"placeholderSecretManager\",\"error\":\"placeholderSecretManager can't be used for address generation or signing\"}"
    );

    let error = ClientError::TransactionBuilder(TransactionBuilderError::InsufficientAmount {
        found: 0,
        required: 100,
    });
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"transactionBuilder\",\"error\":\"insufficient amount: found 0, required 100\"}"
    );

    let error = ClientError::TransactionBuilder(TransactionBuilderError::Block(BlockError::UnsupportedAddressKind(6)));
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"transactionBuilder\",\"error\":\"unsupported address kind: 6\"}"
    );
}
