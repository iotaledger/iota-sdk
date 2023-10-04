// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { utf8ToHex } from '../../../utils/utf8';
import { MetadataFeature } from './feature';

/**
 * The IRC27 NFT standard schema.
 */
class Irc27Metadata {
    /** The IRC standard */
    readonly standard: string = 'IRC27';
    /** The current version. */
    readonly version: string = 'v1.0';
    /** The media type (MIME) of the asset.
     *
     * ## Examples
     * - Image files: `image/jpeg`, `image/png`, `image/gif`, etc.
     * - Video files: `video/x-msvideo` (avi), `video/mp4`, `video/mpeg`, etc.
     * - Audio files: `audio/mpeg`, `audio/wav`, etc.
     * - 3D Assets: `model/obj`, `model/u3d`, etc.
     * - Documents: `application/pdf`, `text/plain`, etc.
     */
    type: string;
    /** URL pointing to the NFT file location. */
    uri: string;
    /** The human-readable name of the native token. */
    name: string;
    /** The human-readable collection name of the native token. */
    collectionName?: string;
    /** Royalty payment addresses mapped to the payout percentage. */
    royalties: Map<string, number> = new Map();
    /** The human-readable name of the native token creator. */
    issuerName?: string;
    /** The human-readable description of the token. */
    description?: string;
    /** Additional attributes which follow [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards). */
    attributes: Attribute[] = [];

    /**
     * @param type The media type (MIME) of the asset.
     * @param uri URL pointing to the NFT file location.
     * @param name The human-readable name of the native token.
     */
    constructor(type: string, uri: string, name: string) {
        this.type = type;
        this.uri = uri;
        this.name = name;
    }

    withCollectionName(collectionName: string): Irc27Metadata {
        this.collectionName = collectionName;
        return this;
    }

    addRoyalty(address: string, percentage: number): Irc27Metadata {
        this.royalties.set(address, percentage);
        return this;
    }

    withRoyalties(royalties: Map<string, number>): Irc27Metadata {
        this.royalties = royalties;
        return this;
    }

    withIssuerName(issuerName: string): Irc27Metadata {
        this.issuerName = issuerName;
        return this;
    }

    withDescription(description: string): Irc27Metadata {
        this.description = description;
        return this;
    }

    addAttribute(attribute: Attribute): Irc27Metadata {
        this.attributes.push(attribute);
        return this;
    }

    withAttributes(attributes: Attribute[]): Irc27Metadata {
        this.attributes = attributes;
        return this;
    }

    asHex(): string {
        return utf8ToHex(JSON.stringify(this));
    }

    asFeature(): MetadataFeature {
        return new MetadataFeature(this.asHex());
    }
}

class Attribute {
    trait_type: string;
    value: any;
    display_type?: string;

    constructor(trait_type: string, value: any) {
        this.trait_type = trait_type;
        this.value = value;
    }

    withDisplayType(display_type: string): Attribute {
        this.display_type = display_type;
        return this;
    }
}

export { Irc27Metadata, Attribute };
