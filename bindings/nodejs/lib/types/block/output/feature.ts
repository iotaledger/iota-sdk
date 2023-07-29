// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../address';

/**
 * All of the feature block types.
 */
enum FeatureType {
    /** TODO. */
    Sender = 0,
    /** TODO. */
    Issuer = 1,
    /** TODO. */
    Metadata = 2,
    /** TODO. */
    Tag = 3,
}

/** TODO. */
abstract class Feature {
    readonly type: FeatureType;

    /** TODO. */
    constructor(type: FeatureType) {
        this.type = type;
    }
    /**
     * Get the type of feature.
     */
    getType(): FeatureType {
        return this.type;
    }
}
/**
 * Sender feature.
 */
class SenderFeature extends Feature {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /** TODO. */
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this.address = sender;
    }
    /**
     * Get the sender address.
     */
    getSender(): Address {
        return this.address;
    }
}
/**
 * Issuer feature.
 */
class IssuerFeature extends Feature {
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    readonly address: Address;

    /** TODO. */
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this.address = issuer;
    }
    /**
     * Get the Issuer address.
     */
    getIssuer(): Address {
        return this.address;
    }
}
/**
 * Metadata feature.
 */
class MetadataFeature extends Feature {
    /** Defines metadata (arbitrary binary data) that will be stored in the output. */
    readonly data: string;
    /** TODO. */
    constructor(data: string) {
        super(FeatureType.Metadata);
        this.data = data;
    }
    /**
     * Get the metadata.
     */
    getData(): string {
        return this.data;
    }
}
/**
 * Tag feature.
 */
class TagFeature extends Feature {
    /** Defines a tag for the data. */
    readonly tag: string;
    /** TODO. */
    constructor(tag: string) {
        super(FeatureType.Tag);
        this.tag = tag;
    }
    /**
     * Get the tag.
     */
    getTag(): string {
        return this.tag;
    }
}
const FeatureDiscriminator = {
    property: 'type',
    subTypes: [
        { value: SenderFeature, name: FeatureType.Sender as any },
        { value: IssuerFeature, name: FeatureType.Issuer as any },
        { value: MetadataFeature, name: FeatureType.Metadata as any },
        { value: TagFeature, name: FeatureType.Tag as any },
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
};
