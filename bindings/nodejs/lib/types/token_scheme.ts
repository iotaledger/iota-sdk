import { HexEncodedAmount } from '@iota/types';

enum TokenSchemeType {
    Simple = 0,
}

abstract class TokenScheme {
    private type: TokenSchemeType;

    constructor(type: TokenSchemeType) {
        this.type = type;
    }

    /**
     * The type of token schem.
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
        mintedTokens: HexEncodedAmount,
        meltedTokens: HexEncodedAmount,
        maximumSupply: HexEncodedAmount,
    ) {
        super(TokenSchemeType.Simple);
        this.mintedTokens = mintedTokens;
        this.meltedTokens = meltedTokens;
        this.maximumSupply = maximumSupply;
    }

    /**
     * Amount of tokens minted.
     */
    getMintedTokens(): HexEncodedAmount {
        return this.mintedTokens;
    }

    /**
     * Amount of tokens melted.
     */
    getMeltedTokens(): HexEncodedAmount {
        return this.meltedTokens;
    }

    /**
     * Maximum supply of tokens controlled.
     */
    getMaximumSupply(): HexEncodedAmount {
        return this.maximumSupply;
    }
}

export { TokenSchemeType, SimpleTokenScheme };
