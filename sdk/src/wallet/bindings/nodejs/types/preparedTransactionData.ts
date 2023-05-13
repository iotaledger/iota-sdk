import type {
    AddressTypes,
    IOutputMetadataResponse,
    ITransactionEssence,
    OutputTypes,
} from '@iota/types';
import { Account, SignedTransactionEssence, Transaction } from '../lib';
import type { Segment } from './output';

/**
 * PreparedTransactionData` is a class that represents prepared transaction data, which
 * is useful for offline signing. It contains the prepared transaction data and an
 * `Account` object. It provides methods to retrieve the prepared transaction data, sign
 * the transaction and sign+submit/finish the transaction.
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
     * The `finish` function returns a promise that resolves to a `Transaction` object after signing
     * and submitting the transaction. Internally just calls `signAndSubmitTransaction`.
     *
     * Returns:
     *
     * The `finish()` method is returning a `Promise` that resolves to a `Transaction` object after it
     * has been signed and submitted.
     */
    public async finish(): Promise<Transaction> {
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

/**
 * Prepared transaction data, useful for offline signing.
 */
export interface IPreparedTransactionData {
    /**
     * Transaction essence
     */
    essence: ITransactionEssence;
    /**
     * Required address information for signing
     */
    inputsData: InputSigningData[];
    /**
     * Optional remainder output information
     */
    remainder?: RemainderData;
}

/**
 * Data for transaction inputs for signing and ordering of unlock blocks
 */
export interface InputSigningData {
    /**
     * The output
     */
    output: OutputTypes;
    /**
     * The output metadata
     */
    outputMetaData: IOutputMetadataResponse;
    /**
     * The chain derived from seed, only for ed25519 addresses
     */
    chain?: Segment[];
    /**
     * The bech32 encoded address, required because of alias outputs where we have multiple possible unlock
     * conditions, because we otherwise don't know which one we need
     */
    bech32Address: string;
}

/**
 * Data for a remainder output, used for ledger nano
 */
export interface RemainderData {
    /**
     * The remainder output
     */
    output: OutputTypes;
    /**
     * The chain derived from seed, for the remainder addresses
     */
    chain?: Segment[];
    /**
     * The remainder address
     */
    address: AddressTypes;
}
