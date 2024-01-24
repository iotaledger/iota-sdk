// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    UnlockConditionDiscriminator,
} from './unlock-condition';
import { Feature, FeatureDiscriminator } from './feature';

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString, NumericString } from '../../utils';
import { TokenScheme, TokenSchemeDiscriminator } from './token-scheme';
import { INativeToken } from '../../models';

export type OutputId = HexEncodedString;

/**
 * All of the output types.
 */
enum OutputType {
    /** A Treasury output. */
    Treasury = 2,
    /** A Basic output. */
    Basic = 3,
    /** An Alias output. */
    Alias = 4,
    /** A Foundry output. */
    Foundry = 5,
    /** An NFT output. */
    Nft = 6,
}

/**
 * The base class for outputs.
 */
abstract class Output /*implements ICommonOutput*/ {
    readonly amount: NumericString;

    readonly type: OutputType;

    /**
     * @param type The type of output.
     * @param amount The amount of the output as big-integer or string.
     */
    constructor(type: OutputType, amount: bigint | NumericString) {
        this.type = type;
        if (typeof amount == 'bigint') {
            this.amount = amount.toString(10);
        } else {
            this.amount = amount;
        }
    }

    /**
     * Get the type of output.
     */
    getType(): OutputType {
        return this.type;
    }

    /**
     * Get the amount of the output.
     */
    getAmount(): bigint {
        return BigInt(this.amount);
    }

    /**
     * Parse an output from a plain JS JSON object.
     */
    public static parse(data: any): Output {
        if (data.type == OutputType.Treasury) {
            return plainToInstance(
                TreasuryOutput,
                data,
            ) as any as TreasuryOutput;
        } else if (data.type == OutputType.Basic) {
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
 * The base class for common outputs.
 */
abstract class CommonOutput extends Output /*implements ICommonOutput*/ {
    @Type(() => UnlockCondition, {
        discriminator: UnlockConditionDiscriminator,
    })
    readonly unlockConditions: UnlockCondition[];

    @Type(() => INativeToken)
    readonly nativeTokens?: INativeToken[];

    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
    readonly features?: Feature[];

    /**
     * @param type The type of output.
     * @param amount The amount of the output.
     * @param unlockConditions Conditions to unlock the output.
     */
    constructor(
        type: OutputType,
        amount: bigint,
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

    /**
     * Get the features contained by the output.
     */
    getFeatures(): Feature[] | undefined {
        return this.features;
    }
}
/**
 * A Treasury output.
 */
class TreasuryOutput extends Output /*implements ITreasuryOutput */ {
    /**
     * @param amount The amount of the output.
     */
    constructor(amount: bigint) {
        super(OutputType.Treasury, amount);
    }
}
/**
 * A Basic output.
 */
class BasicOutput extends CommonOutput /*implements IBasicOutput*/ {
    /**
     * @param amount The amount of the output.
     * @param unlockConditions The unlock conditions for the output.
     */
    constructor(amount: bigint, unlockConditions: UnlockCondition[]) {
        super(OutputType.Basic, amount, unlockConditions);
    }
}

/**
 * Base class for immutable feature outputs.
 */
abstract class ImmutableFeaturesOutput extends CommonOutput {
    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
    readonly immutableFeatures?: Feature[];

    /**
     * @param type The type of output.
     * @param amount The amount of the output.
     * @param unlockConditions The unlock conditions for the output.
     */
    constructor(
        type: OutputType,
        amount: bigint,
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
}

/**
 * Base class for state metadata outputs.
 */
abstract class StateMetadataOutput extends ImmutableFeaturesOutput /*implements IBasicOutput*/ {
    /**
     * Metadata that can only be changed by the state controller.
     */
    readonly stateMetadata?: HexEncodedString;

    /**
     * @param type The type of output.
     * @param amount The amount of the output.
     * @param unlockConditions The unlock conditions for the output.
     * @param stateMetadata Metadata that can only be changed by the state controller.
     */
    constructor(
        type: OutputType,
        amount: bigint,
        unlockConditions: UnlockCondition[],
        stateMetadata?: HexEncodedString,
    ) {
        super(type, amount, unlockConditions);
        this.stateMetadata = stateMetadata;
    }
    /**
     * Metadata that can only be changed by the state controller.
     */
    getStateMetadata(): HexEncodedString | undefined {
        return this.stateMetadata;
    }
}

/**
 * An Alias output.
 */
class AliasOutput extends StateMetadataOutput /*implements IAliasOutput*/ {
    /**
     * Unique identifier of the alias, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its a newly created alias, then the id is zeroed.
     */
    readonly aliasId: HexEncodedString;
    /**
     * A counter that must increase by 1 every time the alias is state transitioned.
     */
    readonly stateIndex: number;
    /**
     * A counter that denotes the number of foundries created by this alias account.
     */
    readonly foundryCounter: number;

    /**
     * @param unlockConditions The unlock conditions of the output.
     * @param amount The amount of the output.
     * @param aliasId The Alias ID as hex-encoded string.
     * @param stateIndex A counter that must increase by 1 every time the alias is state transitioned.
     * @param foundryCounter A counter that denotes the number of foundries created by this alias account.
     * @param stateMetadata Metadata that can only be changed by the state controller.
     */
    constructor(
        unlockConditions: UnlockCondition[],
        amount: bigint,
        aliasId: HexEncodedString,
        stateIndex: number,
        foundryCounter: number,
        stateMetadata?: HexEncodedString,
    ) {
        super(OutputType.Alias, amount, unlockConditions, stateMetadata);
        this.aliasId = aliasId;
        this.stateIndex = stateIndex;
        this.foundryCounter = foundryCounter;
    }
    /**
     * Get the Alias ID of the output.
     */
    getAliasId(): HexEncodedString {
        return this.aliasId;
    }
    /**
     * Get the state index of the output.
     */
    getStateIndex(): number {
        return this.stateIndex;
    }
    /**
     * Get the Foundry counter of the output.
     */
    getFoundryCounter(): number {
        return this.foundryCounter;
    }
}
/**
 * An NFT output.
 */
class NftOutput extends ImmutableFeaturesOutput /*implements INftOutput*/ {
    /**
     * Unique identifier of the NFT, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its newly minted, then the id is zeroed.
     */
    readonly nftId: HexEncodedString;

    /**
     * @param amount The amount of the output.
     * @param nftId The NFT ID as hex-encoded string.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: bigint,
        nftId: HexEncodedString,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Nft, amount, unlockConditions);
        this.nftId = nftId;
    }
    /**
     * Get the NFT ID of the output.
     */
    getNftId(): HexEncodedString {
        return this.nftId;
    }
}
/**
 * A Foundry output.
 */
class FoundryOutput extends ImmutableFeaturesOutput /*implements IFoundryOutput*/ {
    /**
     * The serial number of the Foundry with respect to the controlling alias.
     */
    readonly serialNumber: number;

    /**
     * The token scheme for the Foundry.
     */
    @Type(() => TokenScheme, {
        discriminator: TokenSchemeDiscriminator,
    })
    readonly tokenScheme: TokenScheme;

    /**
     * @param amount The amount of the output.
     * @param serialNumber The serial number of the Foundry with respect to the controlling alias.
     * @param unlockConditions The unlock conditions of the output.
     * @param tokenScheme The token scheme for the Foundry.
     */
    constructor(
        amount: bigint,
        serialNumber: number,
        unlockConditions: UnlockCondition[],
        tokenScheme: TokenScheme,
    ) {
        super(OutputType.Foundry, amount, unlockConditions);
        this.serialNumber = serialNumber;
        this.tokenScheme = tokenScheme;
    }
    /**
     * Get the serial number of the Foundry.
     */
    getSerialNumber(): number {
        return this.serialNumber;
    }
    /**
     * Get the token scheme for the Foundry.
     */
    getTokenScheme(): TokenScheme {
        return this.tokenScheme;
    }
}

const OutputDiscriminator = {
    property: 'type',
    subTypes: [
        { value: TreasuryOutput, name: OutputType.Treasury as any },
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
    TreasuryOutput,
    BasicOutput,
    AliasOutput,
    NftOutput,
    FoundryOutput,
};
