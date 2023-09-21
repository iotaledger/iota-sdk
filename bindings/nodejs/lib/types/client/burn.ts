// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../utils';

/** A DTO for [`Burn`] */
export interface Burn {
    /** Accounts to burn */
    accounts?: string[];
    /** NFTs to burn */
    nfts?: string[];
    /** Foundries to burn */
    foundries?: string[];
    /** Amounts of native tokens to burn */
    nativeTokens?: Map<string, u256>;
}
