// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::VecDeque;

use dialoguer::History;

pub struct AccountHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for AccountHistory {
    fn default() -> Self {
        AccountHistory {
            max: 25,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for AccountHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        let entry = val.to_string();

        // If the last used command is the same, dont change anything
        if matches!(self.history.front(), Some(command) if command == &entry) {
            return;
        }

        // Check if we have used this command before
        match self.history.iter().position(|e| e == &entry) {
            Some(index) => {
                // Remove the old command
                self.history.remove(index);
            }
            None => {
                // We have not used this command
                if self.history.len() == self.max {
                    // Remove the oldest used command
                    self.history.pop_back();
                }
            }
        }

        // Add command as most recent used
        self.history.push_front(entry);
    }
}
