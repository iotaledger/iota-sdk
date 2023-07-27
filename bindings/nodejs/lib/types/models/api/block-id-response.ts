// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { HexEncodedString } from '../../utils/hex-encoding';
/**
 * Block id response.
 */
export interface IBlockIdResponse {
    /**
     * The block id.
     */
    blockId: HexEncodedString;
}
