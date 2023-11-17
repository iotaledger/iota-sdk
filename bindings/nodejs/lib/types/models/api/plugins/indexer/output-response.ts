// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { NumericString } from '../../../../utils';
import type { HexEncodedString } from '../../../../utils/hex-encoding';

/**
 * Details of an outputs response from the indexer plugin.
 */
export interface IOutputsResponse {
    /**
     * The committed slot at which these outputs where available at.
     */
    committedSlot: number;
    /**
     * The maximum amount of items returned in one call. If there are more items, a cursor to the next page is returned too.
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
