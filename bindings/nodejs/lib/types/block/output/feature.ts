// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString, u64 } from '../../utils';
import { Address } from '../address';
import { SlotIndex } from '../slot';

/**
 * All of the feature block types.
 */
enum FeatureType {
    Sender = 0,
    Issuer = 1,
    Metadata = 2,
    Tag = 3,
    BlockIssuer = 4,
    Staking = 5,
}

abstract class Feature {
    readonly type: FeatureType;
    constructor(type: FeatureType) {
        this.type = type;
    }
}

/**
 * Sender feature.
 */
class SenderFeature extends Feature {
    readonly address: Address;
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this.address = sender;
    }
}

/**
 * Issuer feature.
 */
class IssuerFeature extends Feature {
    readonly address: Address;
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this.address = issuer;
    }
}

/**
 * Metadata feature.
 */
class MetadataFeature extends Feature {
    /**
     * Metadata (arbitrary binary data) that will be stored in the output.
     */
    readonly data: string;
    constructor(data: string) {
        super(FeatureType.Metadata);
        this.data = data;
    }
}

/**
 * Tag feature.
 */
class TagFeature extends Feature {
    readonly tag: string;
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
