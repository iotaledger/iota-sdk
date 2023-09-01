// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../../utils';
import { hexToBigInt } from '../../utils/hex-encoding';

/**
 * All of the token scheme types.
 */
enum TokenSchemeType {
    /** A simple token scheme. */
    Simple = 0,
}

/**
 * The base class for token schemes.
 */
abstract class TokenScheme {
    readonly type: TokenSchemeType;

    /**
     * @param type The type of token scheme.
     */
    constructor(type: TokenSchemeType) {
        this.type = type;
    }

    /**
     * Get the type of token scheme.
     */
    getType(): TokenSchemeType {
        return this.type;
    }
}

/**
 * A simple token scheme.
 */
class SimpleTokenScheme extends TokenScheme {
    readonly mintedTokens: u256;
    readonly meltedTokens: u256;
    readonly maximumSupply: u256;

    /**
     * @param mintedTokens The number of tokens that were minted.
     * @param meltedTokens The number of tokens that were melted.
     * @param maximumSupply The maximum supply of the token.
     */
    constructor(mintedTokens: u256, meltedTokens: u256, maximumSupply: u256) {
        super(TokenSchemeType.Simple);
        if (typeof mintedTokens === 'bigint') {
            this.mintedTokens = mintedTokens;
        } else if (mintedTokens) {
            this.mintedTokens = hexToBigInt(mintedTokens);
        } else {
            this.mintedTokens = BigInt(0);
        }

        if (typeof meltedTokens === 'bigint') {
            this.meltedTokens = meltedTokens;
        } else if (meltedTokens) {
            this.meltedTokens = hexToBigInt(meltedTokens);
        } else {
            this.meltedTokens = BigInt(0);
        }

        if (typeof maximumSupply === 'bigint') {
            this.maximumSupply = maximumSupply;
        } else if (maximumSupply) {
            this.maximumSupply = hexToBigInt(maximumSupply);
        } else {
            this.maximumSupply = BigInt(0);
        }
    }

    /**
     * Get the amount of tokens minted.
     */
    getMintedTokens(): u256 {
        return this.mintedTokens;
    }

    /**
     * Get the amount of tokens melted.
     */
    getMeltedTokens(): u256 {
        return this.meltedTokens;
    }

    /**
     * Get the maximum supply of tokens.
     */
    getMaximumSupply(): u256 {
        return this.maximumSupply;
    }
}

const TokenSchemeDiscriminator = {
    property: 'type',
    subTypes: [
        { value: SimpleTokenScheme, name: TokenSchemeType.Simple as any },
    ],
};

export {
    TokenSchemeDiscriminator,
    TokenScheme,
    TokenSchemeType,
    SimpleTokenScheme,
};
