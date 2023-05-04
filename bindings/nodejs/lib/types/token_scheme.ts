import { HexEncodedAmount } from '@iota/types';

enum TokenSchemeType {
    Simple = 0,
}

/**
 * Simple token scheme.
 */
class SimpleTokenScheme {
    private mintedTokens: HexEncodedAmount;

    private meltedTokens: HexEncodedAmount;

    private maximumSupply: HexEncodedAmount;

    private type: TokenSchemeType;

    constructor(
        mintedTokens: HexEncodedAmount,
        meltedTokens: HexEncodedAmount,
        maximumSupply: HexEncodedAmount,
    ) {
        this.mintedTokens = mintedTokens;
        this.meltedTokens = meltedTokens;
        this.maximumSupply = maximumSupply;
        this.type = TokenSchemeType.Simple;
    }

    getType(): TokenSchemeType {
        return this.type;
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
