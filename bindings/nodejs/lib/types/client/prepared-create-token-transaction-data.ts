// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Transaction } from '../wallet';
import { PreparedTransactionData } from './prepared-transaction-data';

/** The result of preparing a minting operation */
export class PreparedCreateNativeTokenTransactionData {
    /** The token id of the minted token */
    tokenId!: string;
    /** The prepared transaction which will mint the token */
    @Type(() => PreparedTransactionData)
    transaction!: PreparedTransactionData;
}

/** The result of a minting operation */
export class CreateNativeTokenTransaction {
    /** The token id of the minted token */
    tokenId!: string;
    /** The transaction which minted the token */
    @Type(() => Transaction)
    transaction!: Transaction;
}
