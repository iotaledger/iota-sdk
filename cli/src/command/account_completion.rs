// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use colored::Colorize;
use rustyline::{
    completion::Completer, highlight::Highlighter, hint::HistoryHinter, Completer, Context, Helper, Hinter, Validator,
};

#[derive(Default)]
pub struct AccountCompleter;

const ACCOUNT_COMMANDS: &[&str] = &[
    "accounts",
    "addresses",
    "balance",
    "burn-native-token",
    "burn-nft",
    "claim",
    "claimable-outputs",
    "clear",
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
    type Candidate = &'static str;

    fn complete(
        &self,
        input: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        Ok((
            0,
            ACCOUNT_COMMANDS
                .iter()
                .filter_map(|cmd| cmd.starts_with(input).then_some(*cmd))
                .collect(),
        ))
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub struct AccountPromptHelper {
    #[rustyline(Completer)]
    completer: AccountCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    prompt: String,
}

impl AccountPromptHelper {
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
}

impl Highlighter for AccountPromptHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(&self.prompt)
        } else {
            Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(hint.bold().to_string())
    }
}

impl Default for AccountPromptHelper {
    fn default() -> Self {
        Self {
            completer: AccountCompleter,
            hinter: HistoryHinter {},
            prompt: String::new(),
        }
    }
}
