import { HexEncodedAmount, SIMPLE_TOKEN_SCHEME_TYPE } from '@iota/types';

enum TokenSchemeType {
    Simple = SIMPLE_TOKEN_SCHEME_TYPE,
}

/**
 * Simple token scheme.
 */
class SimpleTokenScheme {
    private _mintedTokens: HexEncodedAmount;

    private _meltedTokens: HexEncodedAmount;

    private _maximumSupply: HexEncodedAmount;

    constructor(
        mintedTokens: HexEncodedAmount,
        meltedTokens: HexEncodedAmount,
        maximumSupply: HexEncodedAmount,
    ) {
        this._mintedTokens = mintedTokens;
        this._meltedTokens = meltedTokens;
        this._maximumSupply = maximumSupply;
    }

    /**
     * Amount of tokens minted by this foundry.
     */
    getMintedTokens(): HexEncodedAmount {
        return this._mintedTokens;
    }

    /**
     * Amount of tokens melted by this foundry.
     */
    getMeltedTokens(): HexEncodedAmount {
        return this._meltedTokens;
    }

    /**
     * Maximum supply of tokens controlled by this foundry.
     */
    getMaximumSupply(): HexEncodedAmount {
        return this._maximumSupply;
    }
}

export { TokenSchemeType, SimpleTokenScheme };
