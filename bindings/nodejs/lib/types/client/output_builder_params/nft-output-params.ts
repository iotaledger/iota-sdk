// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { Feature, HexEncodedString } from '../..';
import type { BasicOutputBuilderParams } from './basic-output-params';

/**
 * Options for building an Nft Output
 */
export interface NftOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * A unique ID for the new NFT.
     */
    nftId: HexEncodedString;
    /**
     * Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
     */
    immutableFeatures?: Feature[];
}
