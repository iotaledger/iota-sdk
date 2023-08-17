// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { u256 } from '../../utils';
import { hexToBigInt } from '../../utils/hex-encoding';

enum TokenSchemeType {
    Simple = 0,
}

abstract class TokenScheme {
    private type: TokenSchemeType;

    constructor(type: TokenSchemeType) {
        this.type = type;
    }

    /**
     * The type of token scheme.
     */
    getType(): TokenSchemeType {
        return this.type;
    }
}

/**
 * Simple token scheme.
 */
class SimpleTokenScheme extends TokenScheme {
    private mintedTokens: u256;
    private meltedTokens: u256;
    private maximumSupply: u256;

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
     * Amount of tokens minted.
     */
    getMintedTokens(): u256 {
        return this.mintedTokens;
    }

    /**
     * Amount of tokens melted.
     */
    getMeltedTokens(): u256 {
        return this.meltedTokens;
    }

    /**
     * Maximum supply of tokens controlled.
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
