import { HexEncodedAmount } from '@iota/types';

export class NativeToken {
    /**
     * Identifier of the native token.
     */
    id: string;
    /**
     * Amount of native tokens of the given Token ID.
     */
    amount: HexEncodedAmount;

    constructor(id: string, amount: HexEncodedAmount) {
        this.id = id;
        this.amount = amount;
    }
}
