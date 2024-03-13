// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    UnlockCondition,
    UnlockConditionDiscriminator,
} from './unlock-condition';
import { Feature, FeatureDiscriminator, NativeTokenFeature } from './feature';

// Temp solution for not double parsing JSON
import { plainToInstance, Type } from 'class-transformer';
import { HexEncodedString, NumericString, u64 } from '../../utils';
import { TokenScheme, TokenSchemeDiscriminator } from './token-scheme';
import { AccountId, NftId, AnchorId, DelegationId } from '../id';
import { EpochIndex } from '../../block/slot';
import { NativeToken } from '../../models/native-token';
import { Address, AddressDiscriminator, AccountAddress } from '../address';

export type OutputId = HexEncodedString;

/**
 * All of the output types.
 */
enum OutputType {
    /** A Basic output. */
    Basic = 0,
    /** An Account output. */
    Account = 1,
    /** An Anchor output. */
    Anchor = 2,
    /** A Foundry output. */
    Foundry = 3,
    /** An NFT output. */
    Nft = 4,
    /** A Delegation output. */
    Delegation = 5,
}

/**
 * The base class for outputs.
 */
abstract class Output {
    readonly amount: NumericString;

    /**
     * The type of output.
     */
    readonly type: OutputType;

    /**
     * @param type The type of output.
     * @param amount The amount of the output as big-integer or string.
     */
    constructor(type: OutputType, amount: u64 | NumericString) {
        this.type = type;
        if (typeof amount == 'bigint') {
            this.amount = amount.toString(10);
        } else {
            this.amount = amount;
        }
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
        } else if (data.type == OutputType.Anchor) {
            return plainToInstance(AnchorOutput, data) as any as AnchorOutput;
        } else if (data.type == OutputType.Foundry) {
            return plainToInstance(FoundryOutput, data) as any as FoundryOutput;
        } else if (data.type == OutputType.Nft) {
            return plainToInstance(NftOutput, data) as any as NftOutput;
        } else if (data.type == OutputType.Delegation) {
            return plainToInstance(
                DelegationOutput,
                data,
            ) as any as DelegationOutput;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * The base class for common outputs.
 */
abstract class CommonOutput extends Output {
    /**
     * The unlock conditions for the output.
     */
    @Type(() => UnlockCondition, {
        discriminator: UnlockConditionDiscriminator,
    })
    readonly unlockConditions: UnlockCondition[];

    /**
     * The features contained by the output.
     */
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
     * The native token held by the output.
     */
    getNativeToken(): NativeToken | undefined {
        if (!this.features) {
            return undefined;
        }

        for (const feature of this.features) {
            if (feature instanceof NativeTokenFeature) {
                return (feature as NativeTokenFeature).asNativeToken();
            }
        }
        return undefined;
    }
}

/**
 * A Basic output.
 */
class BasicOutput extends CommonOutput {
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
}

/**
 * An Account output.
 */
class AccountOutput extends ImmutableFeaturesOutput {
    /**
     * Unique identifier of the account, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its a newly created account, then the id is zeroed.
     */
    readonly accountId: AccountId;
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
     * @param foundryCounter A counter that denotes the number of foundries created by this account output.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: u64,
        mana: u64,
        accountId: AccountId,
        foundryCounter: number,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Account, amount, unlockConditions);
        this.accountId = accountId;
        this.foundryCounter = foundryCounter;
        this.mana = mana;
    }
}

/**
 * An Anchor output.
 */
class AnchorOutput extends ImmutableFeaturesOutput {
    /**
     * Unique identifier of the anchor, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its a newly created anchor, then the id is zeroed.
     */
    readonly anchorId: AnchorId;
    /**
     * A counter that must increase by 1 every time the anchor output is state transitioned.
     */
    readonly stateIndex: number;
    /**
     * The amount of (stored) Mana held by the output.
     */
    readonly mana: u64;

    /**
     * @param amount The amount of the output.
     * @param mana The amount of stored mana.
     * @param anchorId The anchor ID as hex-encoded string.
     * @param stateIndex A counter that must increase by 1 every time the anchor output is state transitioned.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: u64,
        mana: u64,
        anchorId: AnchorId,
        stateIndex: number,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Account, amount, unlockConditions);
        this.anchorId = anchorId;
        this.stateIndex = stateIndex;
        this.mana = mana;
    }
}

/**
 * A Foundry output.
 */
class FoundryOutput extends ImmutableFeaturesOutput {
    /**
     * The serial number of the Foundry with respect to the controlling account.
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
     * @param serialNumber The serial number of the Foundry with respect to the controlling account.
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
}

/**
 * An NFT output.
 */
class NftOutput extends ImmutableFeaturesOutput {
    /**
     * Unique identifier of the NFT, which is the BLAKE2b-256 hash of the Output ID that created it.
     * Unless its newly minted, then the id is zeroed.
     */
    readonly nftId: NftId;

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
        nftId: NftId,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Nft, amount, unlockConditions);
        this.nftId = nftId;
        this.mana = mana;
    }
}

/**
 * A Delegation output.
 */
class DelegationOutput extends Output {
    /**
     * The amount of delegated coins.
     */
    readonly delegatedAmount: u64;
    /**
     * Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.
     */
    readonly delegationId: DelegationId;
    /**
     * The Account ID of the validator to which this output is delegating.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly validatorAddress: Address;
    /**
     * The index of the first epoch for which this output delegates.
     */
    readonly startEpoch: EpochIndex;
    /**
     * The index of the last epoch for which this output delegates.
     */
    readonly endEpoch: EpochIndex;
    /**
     * The unlock conditions for the output.
     */
    @Type(() => UnlockCondition, {
        discriminator: UnlockConditionDiscriminator,
    })
    readonly unlockConditions: UnlockCondition[];

    /**
     * @param amount The amount of the output.
     * @param delegatedAmount The amount of delegated coins.
     * @param delegationId Unique identifier of the Delegation Output, which is the BLAKE2b-256 hash of the Output ID that created it.
     * @param validatorAddress The Account address of the validator to which this output is delegating.
     * @param startEpoch The index of the first epoch for which this output delegates.
     * @param endEpoch The index of the last epoch for which this output delegates.
     * @param unlockConditions The unlock conditions of the output.
     */
    constructor(
        amount: u64,
        delegatedAmount: u64,
        delegationId: DelegationId,
        validatorAddress: AccountAddress,
        startEpoch: EpochIndex,
        endEpoch: EpochIndex,
        unlockConditions: UnlockCondition[],
    ) {
        super(OutputType.Delegation, amount);
        this.delegatedAmount = delegatedAmount;
        this.delegationId = delegationId;
        this.validatorAddress = validatorAddress;
        this.startEpoch = startEpoch;
        this.endEpoch = endEpoch;
        this.unlockConditions = unlockConditions;
    }
}

const OutputDiscriminator = {
    property: 'type',
    subTypes: [
        { value: BasicOutput, name: OutputType.Basic as any },
        { value: AccountOutput, name: OutputType.Account as any },
        { value: AnchorOutput, name: OutputType.Anchor as any },
        { value: FoundryOutput, name: OutputType.Foundry as any },
        { value: NftOutput, name: OutputType.Nft as any },
        { value: DelegationOutput, name: OutputType.Delegation as any },
    ],
};

export {
    OutputDiscriminator,
    Output,
    OutputType,
    CommonOutput,
    BasicOutput,
    AccountOutput,
    AnchorOutput,
    FoundryOutput,
    NftOutput,
    DelegationOutput,
};
