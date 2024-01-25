// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u64 } from './type-aliases';

/** Decayed stored and potential Mana of an output. */
export interface DecayedMana {
    /** Decayed stored mana. */
    stored: u64;
    /** Decayed potential mana. */
    potential: u64;
}
