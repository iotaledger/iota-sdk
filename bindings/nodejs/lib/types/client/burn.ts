// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../utils';
import { AccountId, FoundryId, NftId, TokenId } from '../block/id';

/** A DTO for [`Burn`] */
export interface Burn {
    /** Accounts to burn */
    accounts?: AccountId[];
    /** NFTs to burn */
    nfts?: NftId[];
    /** Foundries to burn */
    foundries?: FoundryId[];
    /** Amounts of native tokens to burn */
    nativeTokens?: Map<TokenId, u256>;
}
