// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../../utils/hex-encoding';
/**
 * Response from the tips endpoint.
 */
export interface ITipsResponse {
    /**
     * The block ids of the tip.
     */
    tips: HexEncodedString[];
}
