// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/** A DTO for [`Burn`] */
export interface Burn {
    /** Aliases to burn */
    aliases?: string[];
    /** NFTs to burn */
    nfts?: string[];
    /** Foundries to burn */
    foundries?: string[];
    /** Amounts of native tokens to burn */
    nativeTokens?: Map<string, bigint>;
}
