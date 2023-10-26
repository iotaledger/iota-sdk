// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Account,
    PreparedTransactionData,
    SignedTransactionData,
    TransactionWithMetadata,
} from '../..';

/**
 * PreparedTransaction` is a class that represents prepared transaction data, which
 * is useful for offline signing. It contains the prepared transaction data and an
 * `Account` object. It provides methods to retrieve the prepared transaction data, sign
 * the transaction and sign+submit/send the transaction.
 */
export class PreparedTransaction {
    readonly _preparedData: PreparedTransactionData;
    readonly _account: Account;

    /**
     * @param preparedData Prepared data to sign and submit a transaction.
     * @param account A wallet account.
     */
    constructor(preparedData: PreparedTransactionData, account: Account) {
        this._preparedData = preparedData;
        this._account = account;
    }

    /**
     * The function returns the prepared transaction data.
     *
     * Returns:
     *
     * The method `preparedTransactionData()` is returning an object of type
     * `PreparedTransactionData`.
     */
    public preparedTransactionData(): PreparedTransactionData {
        return this._preparedData;
    }

    /**
     * The `send` function returns a promise that resolves to a `Transaction` object after signing
     * and submitting the transaction. Internally just calls `signAndSubmitTransaction`.
     *
     * Returns:
     *
     * The `send()` method is returning a `Promise` that resolves to a `TransactionWithMetadata` object after it
     * has been signed and submitted.
     */
    public async send(): Promise<TransactionWithMetadata> {
        return this.signAndSubmitTransaction();
    }

    /**
     * This function signs a prepared transaction using the account's private key and returns
     * the signed transaction.
     *
     * Returns:
     *
     * A `Promise` that resolves to a `SignedTransactionData` object.
     */
    public async sign(): Promise<SignedTransactionData> {
        return this._account.signTransaction(this.preparedTransactionData());
    }

    /**
     * This function signs and submits a transaction using prepared transaction data.
     *
     * Returns:
     *
     * A Promise that resolves to a TransactionWithMetadata object.
     */
    public async signAndSubmitTransaction(): Promise<TransactionWithMetadata> {
        return this._account.signAndSubmitTransaction(
            this.preparedTransactionData(),
        );
    }
}
