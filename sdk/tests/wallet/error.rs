// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::WalletError;
use pretty_assertions::assert_eq;

#[test]
fn stringified_error() {
    // testing a unit-type-like error
    let error = WalletError::MissingBipPath;
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"missingBipPath\",\"error\":\"missing BIP path\"}"
    );

    // testing a tuple-like error
    let error = WalletError::InvalidMnemonic("nilly willy".to_string());
    assert_eq!(
        serde_json::to_string(&error).unwrap(),
        "{\"type\":\"invalidMnemonic\",\"error\":\"invalid mnemonic: nilly willy\"}"
    );

    // testing a struct-like error
    let error = WalletError::NoOutputsToConsolidate {
        available_outputs: 0,
        consolidation_threshold: 0,
    };
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"noOutputsToConsolidate\",\"error\":\"nothing to consolidate: available outputs: 0, consolidation threshold: 0\"}"
    );
}
