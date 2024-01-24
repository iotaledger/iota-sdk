// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Wallet,
    DelegationId,
    PreparedCreateDelegationTransactionData,
} from '../..';
import { PreparedTransaction } from './prepared-transaction';

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
