// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { INodeInfoBaseToken } from './node-info-base-token';
import type { INodeInfoMetrics } from './node-info-metrics';
import type { ProtocolInfo } from './node-info-protocol';
import type { INodeInfoStatus } from './node-info-status';
/**
 * Response from the /info endpoint.
 */
export interface INodeInfo {
    /**
     * The name of the node.
     */
    name: string;
    /**
     * The semantic version of the node.
     */
    version: string;
    /**
     * The status of the node.
     */
    status: INodeInfoStatus;
    /**
     * The metrics for the node.
     */
    metrics: INodeInfoMetrics;
    /**
     * The protocol parameters.
     */
    protocolParameters: ProtocolInfo[];
    /**
     * The base token info of the node.
     */
    baseToken: INodeInfoBaseToken;
    /**
     * Features supported by the node.
     */
    features: string[];
}
