// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AliasId, FoundryId, NftId, TokenId } from '../block/id';

/** A DTO for [`Burn`] */
export interface Burn {
    /** Aliases to burn */
    aliases?: AliasId[];
    /** NFTs to burn */
    nfts?: NftId[];
    /** Foundries to burn */
    foundries?: FoundryId[];
    /** Amounts of native tokens to burn */
    nativeTokens?: Map<TokenId, bigint>;
}
