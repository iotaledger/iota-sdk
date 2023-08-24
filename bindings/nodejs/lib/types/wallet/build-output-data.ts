// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Feature, INativeToken, TokenScheme, UnlockCondition } from '../';

/** An alias output */
export interface BuildAccountOutputData extends BuildBasicOutputData {
    accountId: string;
    stateIndex?: number;
    stateMetadata?: Uint8Array;
    foundryCounter?: number;
    immutableFeatures?: Feature[];
}

/** A basic output */
export interface BuildBasicOutputData {
    /** If not provided, minimum storage deposit will be used */
    amount?: string;
    nativeTokens?: INativeToken;
    unlockConditions: UnlockCondition[];
    features?: Feature[];
}

/** A foundry output */
export interface BuildFoundryOutputData extends BuildBasicOutputData {
    serialNumber: number;
    tokenScheme: TokenScheme;
    immutableFeatures?: Feature[];
}

/** An nft output */
export interface BuildNftOutputData extends BuildBasicOutputData {
    nftId: string;
    immutableFeatures?: Feature[];
}
