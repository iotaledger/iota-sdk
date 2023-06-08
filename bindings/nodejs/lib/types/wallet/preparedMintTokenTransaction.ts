// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Account, PreparedMintTokenTransactionData } from '../..';

import { PreparedTransaction } from './preparedTransaction';

/*
 * The class PreparedMintTokenTransaction represents prepared data for minting a token transaction.
 */
export class PreparedMintTokenTransaction extends PreparedTransaction {
    private _tokenId: string;

    constructor(
        preparedData: PreparedMintTokenTransactionData,
        account: Account,
    ) {
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
