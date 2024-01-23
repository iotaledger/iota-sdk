// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { TransactionWithMetadata } from './transaction';
import { PreparedTransactionData } from '../client/prepared-transaction-data';
import { DelegationId } from '../block';

/** The result of preparing an operation to create a native token. */
export class PreparedCreateDelegationTransactionData {
    /** The id of the delegation */
    delegationId!: DelegationId;
    /** The prepared transaction which will create the delegation */
    @Type(() => PreparedTransactionData)
    transaction!: PreparedTransactionData;
}

/** The result of an operation to create a native token. */
export class CreateDelegationTransaction {
    /** The id of the delegation */
    delegationId!: DelegationId;
    /** The transaction which created the delegation */
    @Type(() => TransactionWithMetadata)
    transaction!: TransactionWithMetadata;
}
