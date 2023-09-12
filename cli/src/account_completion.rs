// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::Completion;

pub(crate) struct AccountCompletion;

pub(crate) const ACCOUNT_COMPLETION: &[&str] = &[
    "accounts",
    "addresses",
    "balance",
    "burn-native-token",
    "burn-nft",
    "claim",
    "claimable-outputs",
    "consolidate",
    "create-account-output",
    "create-native-token",
    "destroy-account",
    "destroy-foundry",
    "exit",
    "faucet",
    "melt-native-token",
    "mint-native-token",
    "mint-nft",
    "new-address",
    "node-info",
    "output",
    "outputs",
    "send",
    "send-native-token",
    "send-nft",
    "switch",
    "sync",
    "transaction",
    "transactions",
    "tx",
    "txs",
    "unspent-outputs",
    "vote",
    "stop-participating",
    "participation-overview",
    "voting-power",
    "increase-voting-power",
    "decrease-voting-power",
    "voting-output",
    "help",
];

impl Completion for AccountCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = ACCOUNT_COMPLETION
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
