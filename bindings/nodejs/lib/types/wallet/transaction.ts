// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { BlockId, TransactionId } from '../block';
import { SignedTransactionPayload } from '../block/payload/signed_transaction';
import { OutputWithMetadata } from '../models';

/** Possible InclusionStates of transactions sent with the wallet */
export enum InclusionState {
    /** The transaction is pending. */
    Pending = 'Pending',
    /** The transaction is accepted. */
    Accepted = 'Accepted',
    /** The transaction is confirmed. */
    Confirmed = 'Confirmed',
    /** The transaction is finalized. */
    Finalized = 'Finalized',
    /** The transaction is conflicting. */
    Conflicting = 'Conflicting',
    /** The transaction and its in- and outputs are pruned, so it's unknown if it got confirmed or was conflicting. */
    UnknownPruned = 'UnknownPruned',
}

/** A Transaction with metadata */
export class TransactionWithMetadata {
    /** The transaction payload */
    @Type(() => SignedTransactionPayload)
    payload!: SignedTransactionPayload;
    /** The block id in which the transaction payload was included */
    blockId?: BlockId;
    /** The inclusion state of the transaction */
    inclusionState!: InclusionState;
    /** The transaction id */
    transactionId!: TransactionId;
    /** The network id in which the transaction was sent */
    networkId!: string;
    /** If the transaction was created by the wallet or someone else */
    incoming!: boolean;
    /** Note that can be set when sending a transaction and is only stored locally */
    note?: string;
    /**
     * Outputs that are used as input in the transaction.
     * May not be all, because some may have already been deleted from the node.
     */
    @Type(() => OutputWithMetadata)
    inputs!: OutputWithMetadata[];
}
