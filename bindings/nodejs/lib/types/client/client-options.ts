// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import type { IMqttBrokerOptions, INetworkInfo, INode } from './network';

/** Options for the client builder */
export interface IClientOptions {
    /** Node which will be tried first for all requests */
    primaryNode?: string | INode;
    nodes?: Array<string | INode>;
    permanodes?: Array<string | INode>;
    /** If the node health status should be ignored */
    ignoreNodeHealth?: boolean;
    /** Interval in which nodes will be checked for their sync status and the NetworkInfo gets updated */
    nodeSyncInterval?: IDuration;
    /** If node quorum is enabled. Will compare the responses from multiple nodes and only returns the
     * response if quorum_threshold of the nodes return the same one
     */
    quorum?: boolean;
    /** Minimum amount of nodes required for request when quorum is enabled */
    minQuorumSize?: number;
    /** % of nodes that have to return the same response so it gets accepted */
    quorumThreshold?: number;
    /** Data related to the used network */
    networkInfo?: INetworkInfo;
    /** Options for the MQTT broker */
    brokerOptions?: IMqttBrokerOptions;
    /** Timeout for API requests */
    apiTimeout?: IDuration;
}

/** Time duration */
export interface IDuration {
    secs: number;
    nanos: number;
}
