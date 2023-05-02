// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/**
 * All of the output types.
 */
enum OutputType {
    Treasury = TREASURY_OUTPUT_TYPE,
    Basic = BASIC_OUTPUT_TYPE,
    Alias = ALIAS_OUTPUT_TYPE,
    Foundry = FOUNDRY_OUTPUT_TYPE,
    Nft = NFT_OUTPUT_TYPE,
}

import { UnlockCondition } from './unlock_condition';
import { Feature } from './feature';
import {
    ALIAS_OUTPUT_TYPE,
    BASIC_OUTPUT_TYPE,
    FOUNDRY_OUTPUT_TYPE,
    HexEncodedString,
    INativeToken,
    NFT_OUTPUT_TYPE,
    TokenSchemeTypes,
    TREASURY_OUTPUT_TYPE,
} from '@iota/types';

abstract class Output /*implements ICommonOutput*/ {
    private _amount: string;
    private _type: OutputType;

    constructor(type: OutputType, amount: string) {
        this._type = type;
        this._amount = amount;
    }
    /**
     * The type of output.
     */
    get type(): OutputType {
        return this._type;
    }

    /**
     * The amount of IOTA coins to held by the output.
     */
    get amount(): string {
        return this._amount;
    }
}
/**
 * Common output properties.
 */
class CommonOutput extends Output /*implements ICommonOutput*/ {
    private _unlockConditions: UnlockCondition[];
    private _nativeTokens?: INativeToken[];
    private _features?: Feature[];

    constructor(
        type: OutputType,
        amount: string,
        unlockConditions: UnlockCondition[],
    ) {
        super(type, amount);
        this._unlockConditions = unlockConditions;
    }
    /**
     * The unlock conditions for the output.
     */
    get unlockConditions(): UnlockCondition[] {
        return this._unlockConditions;
    }
    /**
     * The native tokens held by the output.
     */
    get nativeTokens(): INativeToken[] | undefined {
        return this._nativeTokens;
    }

    set nativeTokens(tokens: INativeToken[] | undefined) {
        this._nativeTokens = tokens;
    }
    /**
     * Features contained by the output.
     */
    get features(): Feature[] | undefined {
        return this._features;
    }

    set features(features: Feature[] | undefined) {
        this._features = features;
    }
}
/**
 * Treasury Output.
 */
class TreasuryOutput extends Output /*implements ITreasuryOutput */ {
    constructor(amount: string) {
        super(OutputType.Treasury, amount);
    }
}
/**
 * Basic output.
 */
class BasicOutput extends CommonOutput /*implements IBasicOutput*/ {
    constructor(amount: string, unlockConditions: UnlockCondition[]) {
        super(OutputType.Basic, amount, unlockConditions);
    }
}

abstract class ImmutableFeaturesOutput extends CommonOutput {
    private _immutableFeatures?: Feature[];

    constructor(
        type: OutputType,
        amount: string,
        unlockConditions: UnlockCondition[],
    ) {
        super(type, amount, unlockConditions);
    }
    /**
     * Immutable features contained by the output.
     */
    get immutableFeatures(): Feature[] | undefined {
        return this._immutableFeatures;
    }

    set immutableFeatures(immutableFeatures: Feature[] | undefined) {
        this._immutableFeatures = immutableFeatures;
    }
}

abstract class StateMetadataOutput extends ImmutableFeaturesOutput /*implements IBasicOutput*/ {
    private _stateMetadata?: HexEncodedString;

    constructor(
        type: OutputType,
        amount: string,
        unlockConditions: UnlockCondition[],
    ) {
        super(type, amount, unlockConditions);
    }
    /**
     * Metadata that can only be changed by the state controller.
     */
    get stateMetadata(): HexEncodedString | undefined {
        return this._stateMetadata;
    }

    set stateMetadata(stateMetadata: HexEncodedString | undefined) {
        this._stateMetadata = stateMetadata;
    }
}

class AliasOutput extends StateMetadataOutput /*implements IAliasOutput*/ {
    private _aliasId: HexEncodedString;
    private _stateIndex: number;
    private _foundryCounter: number;

    constructor(
        unlockConditions: UnlockCondition[],
        amount: string,
        aliasId: HexEncodedString,
        stateIndex: number,
        foundryCounter: number,
    ) {
        super(OutputType.Alias, amount, unlockConditions);
        this._aliasId = aliasId;
        this._stateIndex = stateIndex;
        this._foundryCounter = foundryCounter;
    }
    /**
     * Unique identifier of the alias, which is the BLAKE2b-160 hash of the Output ID that created it.
     */
    get aliasId(): HexEncodedString {
        return this._aliasId;
    }
    /**
     * A counter that must increase by 1 every time the alias is state transitioned.
     */
    get stateIndex(): number {
        return this._stateIndex;
    }
    /**
     * A counter that denotes the number of foundries created by this alias account.
     */
    get foundryCounter(): number {
        return this._foundryCounter;
    }
}
/**
 * NFT output.
 */
class NftOutput extends StateMetadataOutput /*implements INftOutput*/ {
    private _nftId: HexEncodedString;

    constructor(
        amount: string,
        nftId: HexEncodedString,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Nft, amount, unlockConditions);
        this._nftId = nftId;
    }
    /**
     * Unique identifier of the NFT, which is the BLAKE2b-160 hash of the Output ID that created it.
     */
    get nftId(): HexEncodedString {
        return this._nftId;
    }
}
/**
 * Foundry output.
 */
class FoundryOutput extends ImmutableFeaturesOutput /*implements IFoundryOutput*/ {
    private _serialNumber: number;
    private _tokenScheme: TokenSchemeTypes;

    constructor(
        amount: string,
        serialNumber: number,
        unlockConditions: UnlockCondition[],
        tokenScheme: TokenSchemeTypes,
    ) {
        super(OutputType.Foundry, amount, unlockConditions);
        this._serialNumber = serialNumber;
        this._tokenScheme = tokenScheme;
    }
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    get serialNumber(): number {
        return this._serialNumber;
    }
    /**
     * The token scheme for the foundry.
     */
    get tokenScheme(): TokenSchemeTypes {
        return this._tokenScheme;
    }
}

export {
    OutputType,
    Output,
    TreasuryOutput,
    BasicOutput,
    AliasOutput,
    NftOutput,
    FoundryOutput,
};
