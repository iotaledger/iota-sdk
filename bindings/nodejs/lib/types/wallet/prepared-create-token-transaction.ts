// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Account, PreparedCreateNativeTokenTransactionData } from '../..';

import { PreparedTransaction } from './prepared-transaction';

/*
 * The class PreparedCreateNativeTokenTransaction represents prepared data for issuing a transaction to create a native token.
 */
export class PreparedCreateNativeTokenTransaction extends PreparedTransaction {
    readonly _tokenId: string;

    /**
     * @param preparedData Prepared data to create a Native Token.
     * @param account A wallet account.
     */
    constructor(
        preparedData: PreparedCreateNativeTokenTransactionData,
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
     * The token id of the CreateNativeTokenTransaction.
     */
    public tokenId(): string {
        return this._tokenId;
    }
}
