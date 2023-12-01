// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../../utils/hex-encoding';
/**
 * Details of an output.
 */
export interface IOutputMetadataResponse {
    /**
     * The block id the output was contained in.
     */
    blockId: HexEncodedString;
    /**
     * The transaction id for the output.
     */
    transactionId: HexEncodedString;
    /**
     * The index of the output within the corresponding transaction.
     */
    outputIndex: number;
    /**
     * Tells if the output is spent in a confirmed transaction or not.
     */
    isSpent: boolean;
    /**
     * The current latest commitment id for which the request was made.
     */
    latestCommitmentId: HexEncodedString;
    /**
     * The commitment ID of the slot at which this output was spent.
     */
    commitmentIdSpent?: HexEncodedString;
    /**
     * The transaction this output was spent with.
     */
    transactionIdSpent?: HexEncodedString;
    /**
     * The commitment ID at which the output was included into the ledger.
     */
    includedCommitmentId?: HexEncodedString;
}
