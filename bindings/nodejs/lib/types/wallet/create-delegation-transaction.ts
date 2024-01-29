// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, DelegationId } from '../..';
import { PreparedTransaction } from './prepared-transaction';
import { Type } from 'class-transformer';
import { TransactionWithMetadata } from './transaction';
import { PreparedTransactionData } from '../client/prepared-transaction-data';

/*
 * Represents prepared data for issuing a transaction to create a delegation.
 */
export class PreparedCreateDelegationTransaction extends PreparedTransaction {
    readonly _delegationId: DelegationId;

    /**
     * @param preparedData Prepared data to create a delegation.
     * @param wallet A wallet.
     */
    constructor(
        preparedData: PreparedCreateDelegationTransactionData,
        wallet: Wallet,
    ) {
        super(preparedData.transaction, wallet);
        this._delegationId = preparedData.delegationId;
    }

    /**
     * The function returns the delegationId.
     *
     * @returns: The delegation id of the prepared transaction.
     */
    public delegationId(): DelegationId {
        return this._delegationId;
    }
}

/** The result of preparing an operation to create a delegation. */
export class PreparedCreateDelegationTransactionData {
    /** The id of the delegation */
    delegationId!: DelegationId;
    /** The prepared transaction which will create the delegation */
    @Type(() => PreparedTransactionData)
    transaction!: PreparedTransactionData;
}

/** The result of an operation to create a delegation. */
export class CreateDelegationTransaction {
    /** The id of the delegation */
    delegationId!: DelegationId;
    /** The transaction which created the delegation */
    @Type(() => TransactionWithMetadata)
    transaction!: TransactionWithMetadata;
}
