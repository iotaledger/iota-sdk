// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { UnlockCondition } from './unlock_condition';
import { Feature } from './feature';
import { HexEncodedString, INativeToken, TokenSchemeTypes } from '@iota/types';

/**
 * All of the output types.
 */
enum OutputType {
    Treasury = 2,
    Basic = 3,
    Alias = 4,
    Foundry = 5,
    Nft = 6,
}

abstract class Output /*implements ICommonOutput*/ {
    private amount: string;
    private type: OutputType;

    constructor(type: OutputType, amount: string) {
        this.type = type;
        this.amount = amount;
    }
    /**
     * The type of output.
     */
    getType(): OutputType {
        return this.type;
    }

    /**
     * The amount of the output.
     */
    getAmount(): string {
        return this.amount;
    }
}
/**
 * Common output properties.
 */
class CommonOutput extends Output /*implements ICommonOutput*/ {
    private unlockConditions: UnlockCondition[];
    private nativeTokens?: INativeToken[];
    private features?: Feature[];

    constructor(
        type: OutputType,
        amount: string,
        unlockConditions: UnlockCondition[],
    ) {
        super(type, amount);
        this.unlockConditions = unlockConditions;
    }
    /**
     * The unlock conditions for the output.
     */
    getUnlockConditions(): UnlockCondition[] {
        return this.unlockConditions;
    }
    /**
     * The native tokens held by the output.
     */
    getNativeTokens(): INativeToken[] | undefined {
        return this.nativeTokens;
    }

    setNativeTokens(tokens: INativeToken[] | undefined) {
        this.nativeTokens = tokens;
    }
    /**
     * Features contained by the output.
     */
    getFeatures(): Feature[] | undefined {
        return this.features;
    }

    setFeatures(features: Feature[] | undefined) {
        this.features = features;
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
    private immutableFeatures?: Feature[];

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
    getImmutableFeatures(): Feature[] | undefined {
        return this.immutableFeatures;
    }

    setImmutableFeatures(immutableFeatures: Feature[] | undefined) {
        this.immutableFeatures = immutableFeatures;
    }
}

abstract class StateMetadataOutput extends ImmutableFeaturesOutput /*implements IBasicOutput*/ {
    private stateMetadata?: HexEncodedString;

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
    getStateMetadata(): HexEncodedString | undefined {
        return this.stateMetadata;
    }

    setStateMetadata(stateMetadata: HexEncodedString | undefined) {
        this.stateMetadata = stateMetadata;
    }
}

class AliasOutput extends StateMetadataOutput /*implements IAliasOutput*/ {
    private aliasId: HexEncodedString;
    private stateIndex: number;
    private foundryCounter: number;

    constructor(
        unlockConditions: UnlockCondition[],
        amount: string,
        aliasId: HexEncodedString,
        stateIndex: number,
        foundryCounter: number,
    ) {
        super(OutputType.Alias, amount, unlockConditions);
        this.aliasId = aliasId;
        this.stateIndex = stateIndex;
        this.foundryCounter = foundryCounter;
    }
    /**
     * Unique identifier of the alias, which is the BLAKE2b-160 hash of the Output ID that created it.
     * Unless its a newly created alias, then the id is zeroed.
     */
    getAliasId(): HexEncodedString {
        return this.aliasId;
    }
    /**
     * A counter that must increase by 1 every time the alias is state transitioned.
     */
    getStateIndex(): number {
        return this.stateIndex;
    }
    /**
     * A counter that denotes the number of foundries created by this alias account.
     */
    getFoundryCounter(): number {
        return this.foundryCounter;
    }
}
/**
 * NFT output.
 */
class NftOutput extends StateMetadataOutput /*implements INftOutput*/ {
    private nftId: HexEncodedString;

    constructor(
        amount: string,
        nftId: HexEncodedString,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Nft, amount, unlockConditions);
        this.nftId = nftId;
    }
    /**
     * Unique identifier of the NFT, which is the BLAKE2b-160 hash of the Output ID that created it.
     * Unless its newly minted, then the id is zeroed.
     */
    getNnftId(): HexEncodedString {
        return this.nftId;
    }
}
/**
 * Foundry output.
 */
class FoundryOutput extends ImmutableFeaturesOutput /*implements IFoundryOutput*/ {
    private serialNumber: number;
    private tokenScheme: TokenSchemeTypes;

    constructor(
        amount: string,
        serialNumber: number,
        unlockConditions: UnlockCondition[],
        tokenScheme: TokenSchemeTypes,
    ) {
        super(OutputType.Foundry, amount, unlockConditions);
        this.serialNumber = serialNumber;
        this.tokenScheme = tokenScheme;
    }
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    getSerialNumber(): number {
        return this.serialNumber;
    }
    /**
     * The token scheme for the foundry.
     */
    getTokenScheme(): TokenSchemeTypes {
        return this.tokenScheme;
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
