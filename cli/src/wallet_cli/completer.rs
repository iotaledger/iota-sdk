// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use colored::Colorize;
use rustyline::{
    completion::Completer, highlight::Highlighter, hint::HistoryHinter, Completer, Context, Helper, Hinter, Validator,
};
use strum::VariantNames;

use crate::wallet_cli::WalletCommand;

#[derive(Default)]
pub struct WalletCommandCompleter;

impl Completer for WalletCommandCompleter {
    type Candidate = &'static str;

    fn complete(
        &self,
        input: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        Ok((
            0,
            WalletCommand::VARIANTS
                .iter()
                .filter_map(|cmd| cmd.starts_with(input).then_some(*cmd))
                .collect(),
        ))
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub struct WalletCommandHelper {
    #[rustyline(Completer)]
    completer: WalletCommandCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    prompt: String,
}

impl WalletCommandHelper {
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
}

impl Highlighter for WalletCommandHelper {
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

impl Default for WalletCommandHelper {
    fn default() -> Self {
        Self {
            completer: WalletCommandCompleter,
            hinter: HistoryHinter {},
            prompt: String::new(),
        }
    }
}
