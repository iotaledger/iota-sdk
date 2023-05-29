// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionPayload } from '../block/payload/transaction';
import { IOutputResponse } from '../models/api';
import type { OutputData } from './output';

/** Wallet event types */
export type EventType =
    | '*'
    | 'ConsolidationRequired'
    | 'LedgerAddressGeneration'
    | 'NewOutput'
    | 'SpentOutput'
    | 'TransactionInclusion'
    | 'TransactionProgress';

export type NewOutputEvent = {
    output: OutputData;
    transaction?: TransactionPayload;
    transactionInputs?: IOutputResponse;
};

/** Wallet events */
export enum WalletEvent {
    ConsolidationRequired = 'ConsolidationRequired',
    LedgerAddressGeneration = 'LedgerAddressGeneration',
    NewOutput = 'NewOutput',
    SpentOutput = 'SpentOutput',
    TransactionInclusion = 'TransactionInclusion',
    TransactionProgress = 'TransactionProgress',
}
