// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    UnlockConditionDiscriminator,
} from './unlock-condition';
import { Feature, FeatureDiscriminator } from './feature';

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString } from '../../utils';
import { TokenScheme, TokenSchemeDiscriminator } from './token-scheme';
import { INativeToken } from '../../models';

export type OutputId = string;

/**
 * All of the output types.
 */
enum OutputType {
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

    public static parse(data: any): Output {
        if (data.type == OutputType.Basic) {
            return plainToInstance(BasicOutput, data) as any as BasicOutput;
        } else if (data.type == OutputType.Alias) {
            return plainToInstance(AliasOutput, data) as any as AliasOutput;
        } else if (data.type == OutputType.Foundry) {
            return plainToInstance(FoundryOutput, data) as any as FoundryOutput;
        } else if (data.type == OutputType.Nft) {
            return plainToInstance(NftOutput, data) as any as NftOutput;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * Common output properties.
 */
abstract class CommonOutput extends Output /*implements ICommonOutput*/ {
    @Type(() => UnlockCondition, {
        discriminator: UnlockConditionDiscriminator,
    })
    private unlockConditions: UnlockCondition[];

    private nativeTokens?: INativeToken[];

    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
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
 * Basic output.
 */
class BasicOutput extends CommonOutput /*implements IBasicOutput*/ {
    constructor(amount: string, unlockConditions: UnlockCondition[]) {
        super(OutputType.Basic, amount, unlockConditions);
    }
}

abstract class ImmutableFeaturesOutput extends CommonOutput {
    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
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
class NftOutput extends ImmutableFeaturesOutput /*implements INftOutput*/ {
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
    getNftId(): HexEncodedString {
        return this.nftId;
    }
}
/**
 * Foundry output.
 */
class FoundryOutput extends ImmutableFeaturesOutput /*implements IFoundryOutput*/ {
    private serialNumber: number;

    @Type(() => TokenScheme, {
        discriminator: TokenSchemeDiscriminator,
    })
    private tokenScheme: TokenScheme;

    constructor(
        amount: string,
        serialNumber: number,
        unlockConditions: UnlockCondition[],
        tokenScheme: TokenScheme,
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
    getTokenScheme(): TokenScheme {
        return this.tokenScheme;
    }
}

const OutputDiscriminator = {
    property: 'type',
    subTypes: [
        { value: BasicOutput, name: OutputType.Basic as any },
        { value: AliasOutput, name: OutputType.Alias as any },
        { value: NftOutput, name: OutputType.Nft as any },
        { value: FoundryOutput, name: OutputType.Foundry as any },
    ],
};

export {
    OutputDiscriminator,
    Output,
    OutputType,
    CommonOutput,
    BasicOutput,
    AliasOutput,
    NftOutput,
    FoundryOutput,
};
