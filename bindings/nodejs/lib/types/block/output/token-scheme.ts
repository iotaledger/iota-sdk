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
    private mintedTokens: HexEncodedAmount;
    private meltedTokens: HexEncodedAmount;
    private maximumSupply: HexEncodedAmount;

    constructor(
        mintedTokens: bigint,
        meltedTokens: bigint,
        maximumSupply: bigint,
    ) {
        super(TokenSchemeType.Simple);
        if (typeof mintedTokens === 'bigint') {
            this.mintedTokens = bigIntToHex(mintedTokens);
        } else {
            this.mintedTokens = mintedTokens;
        }

        if (typeof meltedTokens === 'bigint') {
            this.meltedTokens = bigIntToHex(meltedTokens);
        } else {
            this.meltedTokens = meltedTokens;
        }

        if (typeof maximumSupply === 'bigint') {
            this.maximumSupply = bigIntToHex(maximumSupply);
        } else {
            this.maximumSupply = maximumSupply;
        }
    }

    /**
     * Amount of tokens minted.
     */
    getMintedTokens(): bigint {
        return hexToBigInt(this.mintedTokens);
    }

    /**
     * Amount of tokens melted.
     */
    getMeltedTokens(): bigint {
        return hexToBigInt(this.meltedTokens);
    }

    /**
     * Maximum supply of tokens controlled.
     */
    getMaximumSupply(): bigint {
        return hexToBigInt(this.maximumSupply);
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
