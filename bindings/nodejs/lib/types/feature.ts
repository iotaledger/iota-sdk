// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Address } from './address';

enum FeatureType {
    Sender,
    Issuer,
    Metadata,
    Tag,
}

class Feature {
    type: FeatureType;
    constructor(type: FeatureType) {
        this.type = type;
    }
}

class SenderFeature extends Feature {
    private _sender: Address;
    constructor(sender: Address) {
        super(FeatureType.Sender);
        this._sender = sender;
    }
    get sender(): Address {
        return this._sender;
    }
}

class IssuerFeature extends Feature {
    private _issuer: Address;
    constructor(issuer: Address) {
        super(FeatureType.Issuer);
        this._issuer = issuer;
    }
    get issuer(): Address {
        return this._issuer;
    }
}

class MetadataFeature extends Feature {
    private _data: string;
    constructor(data: string) {
        super(FeatureType.Metadata);
        this._data = data;
    }
    get data(): string {
        return this._data;
    }
}

class TagFeature extends Feature {
    private _tag: string;
    constructor(tag: string) {
        super(FeatureType.Tag);
        this._tag = tag;
    }
    get tag(): string {
        return this._tag;
    }
}

export { Feature, SenderFeature, IssuerFeature, MetadataFeature, TagFeature };
