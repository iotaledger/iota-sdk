// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SlotIndex } from '../slot';
import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../address';
import {
    BlockIssuerKey,
    BlockIssuerKeyDiscriminator,
} from './block-issuer-key';
import { u256, u64 } from '../../utils/type-aliases';
import { EpochIndex } from '../../block/slot';
import { INativeToken } from '../../models/native-token';
import { HexEncodedString } from '../../utils/hex-encoding';

/**
 * All of the feature block types.
 */
enum FeatureType {
    /** A Sender feature. */
    Sender = 0,
    /** An Issuer feature. */
    Issuer = 1,
    /** A Metadata feature. */
    Metadata = 2,
    /** A StateMetadata feature. */
    StateMetadata = 3,
    /** A Tag feature. */
    Tag = 4,
    /** A NativeToken feature. */
    NativeToken = 5,
    /** A BlockIssuer feature. */
    BlockIssuer = 6,
    /** A Staking feature. */
    Staking = 7,
}

/** The base class for features. */
abstract class Feature {
    readonly type: FeatureType;

    /**
     * @param type The type of feature.
     */
    constructor(type: FeatureType) {
        this.type = type;
    }
}

/**
 * A Sender feature.
 */
class SenderFeature extends Feature {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /**
     * @param sender The Sender address stored with the feature.
     */
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this.address = sender;
    }
}

/**
 * An Issuer feature.
 */
class IssuerFeature extends Feature {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /**
     * @param issuer The Issuer address stored with the feature.
     */
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this.address = issuer;
    }
}

/**
 * A Metadata feature.
 */
class MetadataFeature extends Feature {
    /** Defines metadata (arbitrary binary data) that will be stored in the output. */
    readonly data: string;

    /**
     * @param data The metadata stored with the feature.
     */
    constructor(data: string) {
        super(FeatureType.Metadata);
        this.data = data;
    }
}

/**
 * A Tag feature.
 */
class TagFeature extends Feature {
    /** Defines a tag for the data. */
    readonly tag: string;

    /**
     * @param tag The tag stored with the feature.
     */
    constructor(tag: string) {
        super(FeatureType.Tag);
        this.tag = tag;
    }
}

/**
 * Native token feature.
 */
class NativeTokenFeature extends Feature {
    /**
     * Identifier of the native token.
     */
    readonly id: HexEncodedString;
    /**
     * Amount of native tokens of the given Token ID.
     */
    readonly amount: u256;

    /**
     * Creates a new `NativeTokenFeature`.
     * @param nativeToken The native token stored with the feature.
     */
    constructor(nativeToken: INativeToken) {
        super(FeatureType.NativeToken);
        this.id = nativeToken.id;
        this.amount = nativeToken.amount;
    }

    /**
     * Returns the native token contained in this feature.
     */
    public asNativeToken(): INativeToken {
        return {
            id: this.id,
            amount: this.amount,
        };
    }
}

/**
 * Block Issuer feature.
 */
class BlockIssuerFeature extends Feature {
    /**
     * The slot index at which the Block Issuer Feature expires and can be removed.
     */
    readonly expirySlot: SlotIndex;
    /**
     * The Block Issuer Keys.
     */
    @Type(() => BlockIssuerKey, {
        discriminator: BlockIssuerKeyDiscriminator,
    })
    readonly blockIssuerKeys: Set<BlockIssuerKey>;

    constructor(expirySlot: SlotIndex, blockIssuerKeys: Set<BlockIssuerKey>) {
        super(FeatureType.BlockIssuer);
        this.expirySlot = expirySlot;
        this.blockIssuerKeys = blockIssuerKeys;
    }
}

/**
 * Staking feature.
 */
class StakingFeature extends Feature {
    /**
     * The amount of coins that are locked and staked in the containing account.
     **/
    readonly stakedAmount: u64;
    /**
     * The fixed cost of the validator, which it receives as part of its Mana rewards.
     */
    readonly fixedCost: u64;
    /**
     * The epoch index in which the staking started.
     */
    readonly startEpoch: EpochIndex;
    /**
     * The epoch index in which the staking ends.
     */
    readonly endEpoch: EpochIndex;

    constructor(
        stakedAmount: u64,
        fixedCost: u64,
        startEpoch: EpochIndex,
        endEpoch: EpochIndex,
    ) {
        super(FeatureType.Staking);
        this.stakedAmount = stakedAmount;
        this.fixedCost = fixedCost;
        this.startEpoch = startEpoch;
        this.endEpoch = endEpoch;
    }
}

const FeatureDiscriminator = {
    property: 'type',
    subTypes: [
        { value: SenderFeature, name: FeatureType.Sender as any },
        { value: IssuerFeature, name: FeatureType.Issuer as any },
        { value: MetadataFeature, name: FeatureType.Metadata as any },
        { value: TagFeature, name: FeatureType.Tag as any },
        { value: NativeTokenFeature, name: FeatureType.NativeToken as any },
        { value: BlockIssuerFeature, name: FeatureType.BlockIssuer as any },
        { value: StakingFeature, name: FeatureType.Staking as any },
    ],
};

export {
    FeatureDiscriminator,
    Feature,
    FeatureType,
    SenderFeature,
    IssuerFeature,
    MetadataFeature,
    TagFeature,
    NativeTokenFeature,
    BlockIssuerFeature,
    StakingFeature,
};
