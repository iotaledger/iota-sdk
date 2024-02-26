// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { NodeInfoBaseToken } from './node-info-base-token';
import type { NodeInfoMetrics } from './node-info-metrics';
import type { ProtocolInfo } from './node-info-protocol';
import type { NodeInfoStatus } from './node-info-status';
/**
 * Response from the /info endpoint.
 */
export interface InfoResponse {
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
    status: NodeInfoStatus;
    /**
     * The metrics for the node.
     */
    metrics: NodeInfoMetrics;
    /**
     * The protocol parameters.
     */
    protocolParameters: ProtocolInfo[];
    /**
     * The base token info of the node.
     */
    baseToken: NodeInfoBaseToken;
}
