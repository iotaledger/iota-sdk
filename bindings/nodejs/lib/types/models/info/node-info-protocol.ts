// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { NumericString } from '../../utils/numeric';
import type { IRent } from '../rent';

/**
 * The Protocol Info.
 */
export interface INodeInfoProtocol {
    /**
     * The human friendly name of the network on which the node operates on.
     */
    networkName: string;
    /**
     * The human readable part of bech32 addresses.
     */
    bech32Hrp: string;
    /**
     * The token supply.
     */
    tokenSupply: NumericString;
    /**
     * The protocol version.
     */
    version: number;
    /**
     * The minimum score required for PoW.
     */
    minPowScore: number;
    /**
     * The rent structure used by given node/network.
     */
    rentStructure: IRent;
}
