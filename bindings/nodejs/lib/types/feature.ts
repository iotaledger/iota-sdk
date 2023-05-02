// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ISSUER_FEATURE_TYPE,
    METADATA_FEATURE_TYPE,
    SENDER_FEATURE_TYPE,
    TAG_FEATURE_TYPE,
} from '@iota/types';
import { Address } from './address';
/**
 * All of the feature block types.
 */
enum FeatureType {
    Sender = SENDER_FEATURE_TYPE,
    Issuer = ISSUER_FEATURE_TYPE,
    Metadata = METADATA_FEATURE_TYPE,
    Tag = TAG_FEATURE_TYPE,
}

abstract class Feature {
    private _type: FeatureType;
    constructor(type: FeatureType) {
        this._type = type;
    }
    /**
     * The type of feature.
     */
    get type(): FeatureType {
        return this._type;
    }
}
/**
 * Sender feature.
 */
class SenderFeature extends Feature {
    private _sender: Address;
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this._sender = sender;
    }
    /**
     * The address.
     */
    get sender(): Address {
        return this._sender;
    }
}
/**
 * Issuer feature.
 */
class IssuerFeature extends Feature {
    private _issuer: Address;
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this._issuer = issuer;
    }
    /**
     * The address.
     */
    get issuer(): Address {
        return this._issuer;
    }
}
/**
 * Metadata feature.
 */
class MetadataFeature extends Feature {
    private _data: string;
    constructor(data: string) {
        super(FeatureType.Metadata);
        this._data = data;
    }
    /**
     * Defines metadata (arbitrary binary data) that will be stored in the output.
     */
    get data(): string {
        return this._data;
    }
}
/**
 * Tag feature.
 */
class TagFeature extends Feature {
    private _tag: string;
    constructor(tag: string) {
        super(FeatureType.Tag);
        this._tag = tag;
    }
    /**
     * Defines a tag for the data.
     */
    get tag(): string {
        return this._tag;
    }
}

export { Feature, SenderFeature, IssuerFeature, MetadataFeature, TagFeature };
