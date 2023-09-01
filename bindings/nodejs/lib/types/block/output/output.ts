// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    UnlockConditionDiscriminator,
} from './unlock-condition';
import { Feature, FeatureDiscriminator } from './feature';

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString, hexToBigInt, u64 } from '../../utils';
import { TokenScheme, TokenSchemeDiscriminator } from './token-scheme';
import { INativeToken } from '../../models';

export type OutputId = string;

/**
 * All of the output types.
 */
enum OutputType {
    /** A Basic output. */
    Basic = 3,
    /** An Account output. */
    Account = 4,
    /** A Foundry output. */
    Foundry = 5,
    /** An NFT output. */
    Nft = 6,
}

/**
 * The base class for outputs.
 */
abstract class Output /*implements ICommonOutput*/ {
    readonly amount: string;

    readonly type: OutputType;

    /**
     * @param type The type of output.
     * @param amount The amount of the output as big-integer or string.
     */
    constructor(type: OutputType, amount: u64 | string) {
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
    getAmount(): u64 {
        return BigInt(this.amount);
    }

    /**
     * Parse an output from a plain JS JSON object.
     */
    public static parse(data: any): Output {
        if (data.type == OutputType.Basic) {
            return plainToInstance(BasicOutput, data) as any as BasicOutput;
        } else if (data.type == OutputType.Account) {
            return plainToInstance(AccountOutput, data) as any as AccountOutput;
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
        amount: u64,
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
     * Get the features contained by the output.
     */
    getFeatures(): Feature[] | undefined {
        return this.features;
    }
}
/**
 * A Basic output.
 */
class BasicOutput extends CommonOutput /*implements IBasicOutput*/ {
    /**
     * The amount of (stored) Mana held by the output.
     */
    readonly mana: u64;

    /**
     * @param amount The amount of the output.
     * @param unlockConditions The unlock conditions for the output.
     */
    constructor(amount: u64, mana: u64, unlockConditions: UnlockCondition[]) {
        super(OutputType.Basic, amount, unlockConditions);
        this.mana = mana;
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
        amount: u64,
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
    readonly stateMetadata?: HexEncodedString;

    /**
     * @param type The type of output.
     * @param amount The amount of the output.
     * @param unlockConditions The unlock conditions for the output.
     */
    constructor(
        type: OutputType,
        amount: u64,
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

/**
 * An Account output.
 */
class AccountOutput extends StateMetadataOutput /*implements IAccountOutput*/ {
    /**
     * Unique identifier of the account, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its a newly created account, then the id is zeroed.
     */
    readonly accountId: HexEncodedString;
    /**
     * A counter that must increase by 1 every time the account output is state transitioned.
     */
    readonly stateIndex: number;
    /**
     * A counter that denotes the number of foundries created by this account output.
     */
    readonly foundryCounter: number;
    /**
     * The amount of (stored) Mana held by the output.
     */
    readonly mana: u64;

    /**
     * @param amount The amount of the output.
     * @param mana The amount of stored mana.
     * @param accountId The account ID as hex-encoded string.
     * @param stateIndex A counter that must increase by 1 every time the account output is state transitioned.
     * @param foundryCounter A counter that denotes the number of foundries created by this account output.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: u64,
        mana: u64,
        accountId: HexEncodedString,
        stateIndex: number,
        foundryCounter: number,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Account, amount, unlockConditions);
        this.accountId = accountId;
        this.stateIndex = stateIndex;
        this.foundryCounter = foundryCounter;
        this.mana = mana;
    }
    /**
     * Unique identifier of the account, which is the BLAKE2b-160 hash of the Output ID that created it.
     * Unless its a newly created account, then the id is zeroed.
     */
    getAccountId(): HexEncodedString {
        return this.accountId;
    }
    /**
     * A counter that must increase by 1 every time the account is state transitioned.
     */
    getStateIndex(): number {
        return this.stateIndex;
    }
    /**
     * A counter that denotes the number of foundries created by this account.
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
     * The amount of (stored) Mana held by the output.
     */
    readonly mana: u64;

    /**
     * @param amount The amount of the output.
     * @param mana The amount of stored mana.
     * @param nftId The NFT ID as hex-encoded string.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: u64,
        mana: u64,
        nftId: HexEncodedString,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Nft, amount, unlockConditions);
        this.nftId = nftId;
        this.mana = mana;
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
        amount: u64,
        serialNumber: number,
        unlockConditions: UnlockCondition[],
        tokenScheme: TokenScheme,
    ) {
        super(OutputType.Foundry, amount, unlockConditions);
        this.serialNumber = serialNumber;
        this.tokenScheme = tokenScheme;
    }
    /**
     * The serial number of the foundry with respect to the controlling account.
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
        { value: BasicOutput, name: OutputType.Basic as any },
        { value: AccountOutput, name: OutputType.Account as any },
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
    AccountOutput,
    NftOutput,
    FoundryOutput,
};
