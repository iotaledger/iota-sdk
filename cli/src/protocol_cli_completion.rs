// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::Completion;

pub(crate) struct ProtocolCliCompletion;

pub(crate) const PROTOCOL_COMMANDS: &[&str] = &[
    "address",
    "balance",
    "burn-native-token",
    "burn-nft",
    "claim",
    "claimable-outputs",
    "clear",
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
    "node-info",
    "output",
    "outputs",
    "send",
    "send-native-token",
    "send-nft",
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

impl Completion for ProtocolCliCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = PROTOCOL_COMMANDS
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
