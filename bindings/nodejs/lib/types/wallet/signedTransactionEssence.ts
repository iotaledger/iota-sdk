// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionPayload } from '../block/payload/transaction';
import { IInputSigningData } from '../client';

/** The signed transaction with inputs data */
export interface SignedTransactionEssence {
    transactionPayload: TransactionPayload;
    inputsData: IInputSigningData;
}
