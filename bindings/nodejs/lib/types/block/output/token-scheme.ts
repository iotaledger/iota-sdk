// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../../utils';
import { Transform } from 'class-transformer';
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
    /**
     * The type of token scheme.
     */
    readonly type: TokenSchemeType;

    /**
     * @param type The type of token scheme.
     */
    constructor(type: TokenSchemeType) {
        this.type = type;
    }
}

/**
 * A simple token scheme.
 */
class SimpleTokenScheme extends TokenScheme {
    /**
     * The amount of tokens minted.
     */
    @Transform((value) => hexToBigInt(value.value))
    readonly mintedTokens: u256;

    /**
     * The amount of tokens melted.
     */
    @Transform((value) => hexToBigInt(value.value))
    readonly meltedTokens: u256;

    /**
     * The maximum supply of tokens.
     */
    @Transform((value) => hexToBigInt(value.value))
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
