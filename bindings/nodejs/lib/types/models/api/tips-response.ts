// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../../block/id';

/**
 * Response from the tips endpoint.
 */
export interface ITipsResponse {
    /**
     * The block ids of the tip.
     */
    tips: BlockId[];
}
