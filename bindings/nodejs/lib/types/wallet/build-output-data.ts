// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Feature, INativeToken, TokenScheme, UnlockCondition } from '../';

/** @deprecated use AliasOutputBuilderParams instead. */
export interface BuildAliasOutputData extends BuildBasicOutputData {
    /** TODO */
    aliasId: string;
    /** TODO */
    stateIndex?: number;
    /** TODO */
    stateMetadata?: Uint8Array;
    /** TODO */
    foundryCounter?: number;
    /** TODO */
    immutableFeatures?: Feature[];
}

/** @deprecated use BasicOutputBuilderParams instead. */
export interface BuildBasicOutputData {
    /** If not provided, minimum storage deposit will be used */
    amount?: string;
    /** TODO */
    nativeTokens?: INativeToken;
    /** TODO */
    unlockConditions: UnlockCondition[];
    /** TODO */
    features?: Feature[];
}

/** @deprecated use FoundryOutputBuilderParams instead. */
export interface BuildFoundryOutputData extends BuildBasicOutputData {
    /** TODO */
    serialNumber: number;
    /** TODO */
    tokenScheme: TokenScheme;
    /** TODO */
    immutableFeatures?: Feature[];
}

/** @deprecated use NftOutputBuilderParams instead. */
export interface BuildNftOutputData extends BuildBasicOutputData {
    /** TODO */
    nftId: string;
    /** TODO */
    immutableFeatures?: Feature[];
}
