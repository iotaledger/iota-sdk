import type {
    INativeToken,
    TokenSchemeTypes,
} from '@iota/types';
import { Feature, UnlockCondition } from '../../lib';

/** An alias output */
export interface BuildAliasOutputData extends BuildBasicOutputData {
    aliasId: string;
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
    tokenScheme: TokenSchemeTypes;
    immutableFeatures?: Feature[];
}

/** An nft output */
export interface BuildNftOutputData extends BuildBasicOutputData {
    nftId: string;
    immutableFeatures?: Feature[];
}
