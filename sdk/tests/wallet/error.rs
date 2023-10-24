// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::wallet::Error;
use pretty_assertions::assert_eq;

#[test]
fn stringified_error() {
    let error = Error::NoOutputsToConsolidate {
        available_outputs: 0,
        consolidation_threshold: 0,
    };
    assert_eq!(
        &serde_json::to_string(&error).unwrap(),
        "{\"type\":\"noOutputsToConsolidate\",\"error\":\"nothing to consolidate: available outputs: 0, consolidation threshold: 0\"}"
    );
}
