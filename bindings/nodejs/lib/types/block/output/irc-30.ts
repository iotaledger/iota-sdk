// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { utf8ToHex } from '../../../utils/utf8';
import { MetadataFeature } from './feature';

/**
 * The IRC30 native token metadata standard schema.
 */
class Irc30Metadata {
    /** The IRC standard */
    readonly standard: string = 'IRC30';
    /** The human-readable name of the native token. */
    name: string;
    /** The symbol/ticker of the token. */
    symbol: string;
    /** Number of decimals the token uses (divide the token amount by `10^decimals` to get its user representation). */
    decimals: number;
    /** The human-readable description of the token. */
    description?: string;
    /** URL pointing to more resources about the token. */
    url?: string;
    /** URL pointing to an image resource of the token logo. */
    logoUrl?: string;
    /** The svg logo of the token encoded as a byte string. */
    logo?: string;

    /**
     * @param name The human-readable name of the native token.
     * @param symbol The symbol/ticker of the token.
     * @param decimals Number of decimals the token uses.
     */
    constructor(name: string, symbol: string, decimals: number) {
        this.name = name;
        this.symbol = symbol;
        this.decimals = decimals;
    }

    withDescription(description: string): Irc30Metadata {
        this.description = description;
        return this;
    }

    withUrl(url: string): Irc30Metadata {
        this.url = url;
        return this;
    }

    withLogoUrl(logoUrl: string): Irc30Metadata {
        this.logoUrl = logoUrl;
        return this;
    }

    withLogo(logo: string): Irc30Metadata {
        this.logo = logo;
        return this;
    }

    asHex(): string {
        return utf8ToHex(JSON.stringify(this));
    }

    asFeature(): MetadataFeature {
        return new MetadataFeature(this.asHex());
    }
}

export { Irc30Metadata };
