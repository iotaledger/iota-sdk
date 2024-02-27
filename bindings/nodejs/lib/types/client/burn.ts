// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../utils';
import { AccountId, FoundryId, NftId, TokenId } from '../block/id';

/** A DTO for [`Burn`] */
export interface Burn {
    /** Burn initial excess mana (only from inputs/outputs that have been specified manually) */
    mana?: boolean;
    /** Burn generated mana */
    generatedMana?: boolean;
    /** Accounts to burn */
    accounts?: AccountId[];
    /** NFTs to burn */
    nfts?: NftId[];
    /** Foundries to burn */
    foundries?: FoundryId[];
    /** Amounts of native tokens to burn */
    nativeTokens?: Map<TokenId, u256>;
}
