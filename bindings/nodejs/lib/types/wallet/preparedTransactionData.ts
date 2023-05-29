import {
    Account,
    IPreparedTransactionData,
    SignedTransactionEssence,
    Transaction,
} from '../../';

/**
 * PreparedTransactionData` is a class that represents prepared transaction data, which
 * is useful for offline signing. It contains the prepared transaction data and an
 * `Account` object. It provides methods to retrieve the prepared transaction data, sign
 * the transaction and sign+submit/send the transaction.
 */
export class PreparedTransactionData {
    private _preparedData: IPreparedTransactionData;
    private _account: Account;

    constructor(preparedData: IPreparedTransactionData, account: Account) {
        this._preparedData = preparedData;
        this._account = account;
    }

    /**
     * The function returns the prepared transaction data.
     *
     * Returns:
     *
     * The method `preparedTransactionData()` is returning an object of type
     * `IPreparedTransactionData`.
     */
    public preparedTransactionData(): IPreparedTransactionData {
        return this._preparedData;
    }

    /**
     * The `send` function returns a promise that resolves to a `Transaction` object after signing
     * and submitting the transaction. Internally just calls `signAndSubmitTransaction`.
     *
     * Returns:
     *
     * The `send()` method is returning a `Promise` that resolves to a `Transaction` object after it
     * has been signed and submitted.
     */
    public async send(): Promise<Transaction> {
        return this.signAndSubmitTransaction();
    }

    /**
     * This function signs a prepared transaction essence using the account's private key and returns
     * the signed transaction essence.
     *
     * Returns:
     *
     * A `Promise` that resolves to a `SignedTransactionEssence` object.
     */
    public async sign(): Promise<SignedTransactionEssence> {
        return this._account.signTransactionEssence(
            this.preparedTransactionData(),
        );
    }

    /**
     * This function signs and submits a transaction using prepared transaction data.
     *
     * Returns:
     *
     * A Promise that resolves to a Transaction object.
     */
    public async signAndSubmitTransaction(): Promise<Transaction> {
        return this._account.signAndSubmitTransaction(
            this.preparedTransactionData(),
        );
    }
}
