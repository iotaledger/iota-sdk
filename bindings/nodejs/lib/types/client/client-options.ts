// Copyright 2021-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ProtocolParameters } from '../models/api/node/info-response';
import type { MqttBrokerOptions, Node } from './network';

/** Options for the client builder */
export interface ClientOptions {
    /** Nodes which will be tried first for all requests */
    primaryNodes?: Array<string | Node>;
    /** A list of nodes. */
    nodes?: Array<string | Node>;
    /** If the node health status should be ignored */
    ignoreNodeHealth?: boolean;
    /** Interval in which nodes will be checked for their sync status and the network info gets updated */
    nodeSyncInterval?: Duration;
    /** If node quorum is enabled. Will compare the responses from multiple nodes and only returns the
     * response if quorum_threshold of the nodes return the same one
     */
    quorum?: boolean;
    /** Minimum amount of nodes required for request when quorum is enabled */
    minQuorumSize?: number;
    /** % of nodes that have to return the same response so it gets accepted */
    quorumThreshold?: number;
    /** The User-Agent header for requests */
    userAgent?: string;
    /** Options for the MQTT broker */
    brokerOptions?: MqttBrokerOptions;
    /** Protocol parameters */
    protocolParameters?: ProtocolParameters;
    /** Timeout for API requests */
    apiTimeout?: Duration;
    /** The maximum parallel API requests. */
    maxParallelApiRequests?: number;
}

/** Time duration */
export interface Duration {
    /** Seconds. */
    secs: number;
    /** Nanoseconds. */
    nanos: number;
}
