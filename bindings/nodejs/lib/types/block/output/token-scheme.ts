// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedAmount } from '../../utils';
import { bigIntToHex, hexToBigInt } from '../../utils/hex-encoding';

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
    private mintedTokens: bigint;
    private meltedTokens: bigint;
    private maximumSupply: bigint;

    constructor(
        mintedTokens: bigint,
        meltedTokens: bigint,
        maximumSupply: bigint,
    ) {
        super(TokenSchemeType.Simple);
        if (typeof mintedTokens === 'bigint') {
            this.mintedTokens = mintedTokens;
        } else {
            this.mintedTokens = hexToBigInt(mintedTokens);
        }

        if (typeof meltedTokens === 'bigint') {
            this.meltedTokens = meltedTokens;
        } else {
            this.meltedTokens = hexToBigInt(meltedTokens);
        }

        if (typeof maximumSupply === 'bigint') {
            this.maximumSupply = maximumSupply;
        } else {
            this.maximumSupply = hexToBigInt(maximumSupply);
        }
    }

    /**
     * Amount of tokens minted.
     */
    getMintedTokens(): bigint {
        return this.mintedTokens;
    }

    /**
     * Amount of tokens melted.
     */
    getMeltedTokens(): bigint {
        return this.meltedTokens;
    }

    /**
     * Maximum supply of tokens controlled.
     */
    getMaximumSupply(): bigint {
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
