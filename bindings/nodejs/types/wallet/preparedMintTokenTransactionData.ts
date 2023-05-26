import { Account, IPreparedTransactionData, Transaction } from '../../lib';

import { PreparedTransactionData } from './preparedTransactionData';

/*
 * The class PreparedMintTokenTransactionData represents prepared data for minting a token transaction.
 */
export class PreparedMintTokenTransactionData extends PreparedTransactionData {
    private _tokenId: string;

    constructor(preparedData: PreparedMintTokenTransaction, account: Account) {
        super(preparedData.transaction, account);
        this._tokenId = preparedData.tokenId;
    }

    /**
     * The function returns the tokenId as a string.
     *
     * Returns:
     *
     * The token id of the MintTokenTransaction.
     */
    public tokenId(): string {
        return this._tokenId;
    }
}

/** The result of preparing a minting operation */
export interface PreparedMintTokenTransaction {
    /** The token id of the minted token */
    tokenId: string;
    /** The prepared transaction which will mint the token */
    transaction: IPreparedTransactionData;
}

/** The result of a minting operation */
export interface MintTokenTransaction {
    /** The token id of the minted token */
    tokenId: string;
    /** The transaction which minted the token */
    transaction: Transaction;
}
