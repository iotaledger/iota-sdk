// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    UnlockConditionDiscriminator,
} from './unlock-condition';
import { Feature, FeatureDiscriminator } from './feature';

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString, hexToBigInt } from '../../utils';
import { TokenScheme, TokenSchemeDiscriminator } from './token-scheme';
import { INativeToken } from '../../models';

export type OutputId = string;

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
    readonly amount: string;

    readonly type: OutputType;

    constructor(type: OutputType, amount: bigint | string) {
        this.type = type;
        if (typeof amount == 'bigint') {
            this.amount = amount.toString(10);
        } else {
            this.amount = amount;
        }
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
    getAmount(): bigint {
        return BigInt(this.amount);
    }

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
 * Common output properties.
 */
abstract class CommonOutput extends Output /*implements ICommonOutput*/ {
    @Type(() => UnlockCondition, {
        discriminator: UnlockConditionDiscriminator,
    })
    readonly unlockConditions: UnlockCondition[];

    readonly nativeTokens?: INativeToken[];

    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
    readonly features?: Feature[];

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
        if (!this.nativeTokens) {
            return this.nativeTokens;
        }

        // Make sure the amount of native tokens are of bigint type.
        for (let i = 0; i < this.nativeTokens.length; i++) {
            const token = this.nativeTokens[i];
            if (typeof token.amount === 'string') {
                this.nativeTokens[i].amount = hexToBigInt(token.amount);
            }
        }
        return this.nativeTokens;
    }

    /**
     * Features contained by the output.
     */
    getFeatures(): Feature[] | undefined {
        return this.features;
    }
}
/**
 * Treasury Output.
 */
class TreasuryOutput extends Output /*implements ITreasuryOutput */ {
    constructor(amount: bigint) {
        super(OutputType.Treasury, amount);
    }
}
/**
 * Basic output.
 */
class BasicOutput extends CommonOutput /*implements IBasicOutput*/ {
    constructor(amount: bigint, unlockConditions: UnlockCondition[]) {
        super(OutputType.Basic, amount, unlockConditions);
    }
}

abstract class ImmutableFeaturesOutput extends CommonOutput {
    @Type(() => Feature, {
        discriminator: FeatureDiscriminator,
    })
    readonly immutableFeatures?: Feature[];

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

abstract class StateMetadataOutput extends ImmutableFeaturesOutput /*implements IBasicOutput*/ {
    readonly stateMetadata?: HexEncodedString;

    constructor(
        type: OutputType,
        amount: bigint,
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
}

class AliasOutput extends StateMetadataOutput /*implements IAliasOutput*/ {
    readonly aliasId: HexEncodedString;
    readonly stateIndex: number;
    readonly foundryCounter: number;

    constructor(
        unlockConditions: UnlockCondition[],
        amount: bigint,
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
    readonly nftId: HexEncodedString;

    constructor(
        amount: bigint,
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
    readonly serialNumber: number;

    @Type(() => TokenScheme, {
        discriminator: TokenSchemeDiscriminator,
    })
    readonly tokenScheme: TokenScheme;

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
