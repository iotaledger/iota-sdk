// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../address';

/**
 * All of the feature block types.
 */
enum FeatureType {
    Sender = 0,
    Issuer = 1,
    Metadata = 2,
    Tag = 3,
}

abstract class Feature {
    private type: FeatureType;
    constructor(type: FeatureType) {
        this.type = type;
    }
    /**
     * The type of feature.
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
    private address: Address;
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this.address = sender;
    }
    /**
     * The address.
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
    private address: Address;
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this.address = issuer;
    }
    /**
     * The address.
     */
    getIssuer(): Address {
        return this.address;
    }
}
/**
 * Metadata feature.
 */
class MetadataFeature extends Feature {
    private data: string;
    constructor(data: string) {
        super(FeatureType.Metadata);
        this.data = data;
    }
    /**
     * Defines metadata (arbitrary binary data) that will be stored in the output.
     */
    getData(): string {
        return this.data;
    }
}
/**
 * Tag feature.
 */
class TagFeature extends Feature {
    private tag: string;
    constructor(tag: string) {
        super(FeatureType.Tag);
        this.tag = tag;
    }
    /**
     * Defines a tag for the data.
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
