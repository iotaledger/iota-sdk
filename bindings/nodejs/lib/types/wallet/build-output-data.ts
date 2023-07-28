// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Feature, INativeToken, TokenScheme, UnlockCondition } from '../';

/** @deprecated use AliasOutputBuilderParams instead. */
export interface BuildAliasOutputData extends BuildBasicOutputData {
    aliasId: string;
    stateIndex?: number;
    stateMetadata?: Uint8Array;
    foundryCounter?: number;
    immutableFeatures?: Feature[];
}

/** @deprecated use BasicOutputBuilderParams instead. */
export interface BuildBasicOutputData {
    /** If not provided, minimum storage deposit will be used */
    amount?: string;
    nativeTokens?: INativeToken;
    unlockConditions: UnlockCondition[];
    features?: Feature[];
}

/** @deprecated use FoundryOutputBuilderParams instead. */
export interface BuildFoundryOutputData extends BuildBasicOutputData {
    serialNumber: number;
    tokenScheme: TokenScheme;
    immutableFeatures?: Feature[];
}

/** @deprecated use NftOutputBuilderParams instead. */
export interface BuildNftOutputData extends BuildBasicOutputData {
    nftId: string;
    immutableFeatures?: Feature[];
}
