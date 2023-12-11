// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection};
use crate::client::secret::types::InputSigningData;

impl InputSelection {
    pub(crate) fn fulfill_mana_requirement(&mut self) -> Result<Vec<InputSigningData>, Error> {
        Ok(vec![])
    }
}
