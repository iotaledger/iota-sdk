// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '../..';
import { PreparedTransaction } from './prepared-transaction';
import { Type } from 'class-transformer';
import { TransactionWithMetadata } from './transaction';
import { PreparedTransactionData } from '../client/prepared-transaction-data';

/*
 * The class PreparedCreateNativeTokenTransaction represents prepared data for issuing a transaction to create a native token.
 */
export class PreparedCreateNativeTokenTransaction extends PreparedTransaction {
    readonly _tokenId: string;

    /**
     * @param preparedData Prepared data to create a Native Token.
     * @param wallet A wallet.
     */
    constructor(
        preparedData: PreparedCreateNativeTokenTransactionData,
        wallet: Wallet,
    ) {
        super(preparedData.transaction, wallet);
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

/** The result of preparing an operation to create a native token. */
export class PreparedCreateNativeTokenTransactionData {
    /** The token id of the minted token */
    tokenId!: string;
    /** The prepared transaction which will mint the token */
    @Type(() => PreparedTransactionData)
    transaction!: PreparedTransactionData;
}

/** The result of an operation to create a native token. */
export class CreateNativeTokenTransaction {
    /** The token id of the minted token */
    tokenId!: string;
    /** The transaction which minted the token */
    @Type(() => TransactionWithMetadata)
    transaction!: TransactionWithMetadata;
}
