// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rustyline::{completion::Completer, Context};

pub struct AccountCompleter;

const ACCOUNT_COMMANDS: &[&str] = &[
    "accounts",
    "addresses",
    "balance",
    "burn-native-token",
    "burn-nft",
    "claim",
    "claimable-outputs",
    "consolidate",
    "create-alias-output",
    "create-native-token",
    "destroy-alias",
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

impl Completer for AccountCompleter {
    type Candidate = String;
    fn complete(&self, input: &str, _pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<String>)> {
        let mut completions = vec![];
        for command in ACCOUNT_COMMANDS {
            if command.starts_with(input) {
                completions.push(command.to_string());
            }
        }
        Ok((0, completions))
    }
}
