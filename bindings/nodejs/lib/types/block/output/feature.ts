// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../utils';
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
 * Block Issuer feature
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

    constructor(expirtySlot: SlotIndex, publicKeys: Set<HexEncodedString>) {
        super(FeatureType.BlockIssuer);
        this.expirySlot = expirtySlot;
        this.publicKeys = publicKeys;
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
};
