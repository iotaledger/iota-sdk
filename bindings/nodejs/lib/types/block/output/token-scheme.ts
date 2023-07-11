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
        mintedTokens: BigInt,
        meltedTokens: BigInt,
        maximumSupply: BigInt,
    ) {
        super(TokenSchemeType.Simple);
        this.mintedTokens = bigIntToHex(mintedTokens);
        this.meltedTokens = bigIntToHex(meltedTokens);
        this.maximumSupply = bigIntToHex(maximumSupply);
    }

    /**
     * Amount of tokens minted.
     */
    getMintedTokens(): BigInt {
        return hexToBigInt(this.mintedTokens);
    }

    /**
     * Amount of tokens melted.
     */
    getMeltedTokens(): BigInt {
        return hexToBigInt(this.meltedTokens);
    }

    /**
     * Maximum supply of tokens controlled.
     */
    getMaximumSupply(): BigInt {
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
