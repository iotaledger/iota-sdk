// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { NumericString } from '../../../../utils';
import type { HexEncodedString } from '../../../../utils/hex-encoding';

/**
 * Details of an outputs response from the indexer plugin.
 */
export interface IOutputsResponse {
    /**
     * The ledger index at which these outputs where available at.
     */
    ledgerIndex: number;
    /**
     * The maximum count of results that are returned by the node.
     */
    pageSize: NumericString;
    /**
     * The cursor to use for getting the next results.
     */
    cursor?: string;
    /**
     * The output IDs (transaction hash + output index) of the outputs on this address.
     */
    items: HexEncodedString[];
}
