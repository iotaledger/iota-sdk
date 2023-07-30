// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { TransactionPayload } from '../block/payload/transaction';
import { InputSigningData } from '../client';

/** The signed transaction with inputs data */
export class SignedTransactionEssence {
    /** A transaction payload. */
    @Type(() => TransactionPayload)
    transactionPayload!: TransactionPayload;
    /** Signed inputs data. */
    @Type(() => InputSigningData)
    inputsData!: InputSigningData;
}
