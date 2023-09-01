// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString, u64 } from '../../utils';
import { SlotIndex } from '../slot';
import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../address';

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
    /** A Tag feature. */
    Tag = 3,
    BlockIssuer = 4,
    Staking = 5,
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
    readonly publicKeys: Set<HexEncodedString>;

    constructor(expirySlot: SlotIndex, publicKeys: Set<HexEncodedString>) {
        super(FeatureType.BlockIssuer);
        this.expirySlot = expirySlot;
        this.publicKeys = publicKeys;
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
    readonly startEpoch: u64;
    /**
     * The epoch index in which the staking ends.
     */
    readonly endEpoch: u64;

    constructor(
        stakedAmount: u64,
        fixedCost: u64,
        startEpoch: u64,
        endEpoch: u64,
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
    BlockIssuerFeature,
    StakingFeature,
};
