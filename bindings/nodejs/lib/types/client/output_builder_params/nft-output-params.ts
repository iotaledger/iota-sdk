// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Feature, NftId } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Nft Output
 */
export interface NftOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * Unique identifier of an NFT, which is the BLAKE2b-256 hash of the Output ID that created it.
     */
    nftId: NftId;
    /**
     * Features that add utility to the output but do not impose unlocking conditions.
     * These features need to be kept in future transitions of the UTXO state machine.
     */
    immutableFeatures?: Feature[];
}
