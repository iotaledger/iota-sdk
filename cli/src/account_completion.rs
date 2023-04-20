// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use dialoguer::Completion;

pub(crate) struct AccountCompletion<'a> {
    options: [&'a str; 35],
}

pub(crate) const ACCOUNT_COMPLETION: AccountCompletion = AccountCompletion {
    options: [
        "addresses",
        "balance",
        "burn-native-token",
        "burn-nft",
        "claim",
        "consolidate",
        "create-alias-output",
        "decrease-native-token-supply",
        "destroy-alias",
        "destroy-foundry",
        "exit",
        "faucet",
        "increase-native-token-supply",
        "mint-native-token",
        "mint-nft",
        "new-address",
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
    ],
};

impl<'a> Completion for AccountCompletion<'a> {
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
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
