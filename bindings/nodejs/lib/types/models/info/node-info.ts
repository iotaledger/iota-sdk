// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    INodeInfoBaseToken,
    INodeInfoMetrics,
    INodeInfoProtocol,
    NodeInfoProtocolParamsMilestoneOpt,
    INodeInfoStatus,
} from './internal';
/**
 * Response from the /info endpoint.
 */
export interface INodeInfo {
    /**
     * The name of the node.
     */
    name: string;
    /**
     * The version of node.
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
     * The supported protocol versions.
     */
    supportedProtocolVersions: number[];
    /**
     * The protocol info of the node.
     */
    protocol: INodeInfoProtocol;
    /**
     * Pending protocol parameters.
     */
    pendingProtocolParameters: NodeInfoProtocolParamsMilestoneOpt[];
    /**
     * The base token info of the node.
     */
    baseToken: INodeInfoBaseToken;
    /**
     * Features supported by the node.
     */
    features: string[];
}
