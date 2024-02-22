// Copyright 2021-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ProtocolParameters } from '../models/info/node-info-protocol';
import type { INode } from './network';

/** Options for the client builder */
export interface IClientOptions {
    /** Nodes which will be tried first for all requests */
    primaryNodes?: Array<string | INode>;
    /** A list of nodes. */
    nodes?: Array<string | INode>;
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
    /** The User-Agent header for requests */
    userAgent?: string;
    /** Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not. */
    automaticDisconnect?: boolean;
    /** Sets the timeout in seconds used for the MQTT operations. */
    timeout?: number;
    /** Sets whether websockets should be used instead of regular TCP for the MQTT operations. */
    useWs?: boolean;
    /** Sets the port used for the MQTT operations. */
    port?: number;
    /** Sets the maximum number of reconnection attempts. 0 is unlimited. */
    maxReconnectionAttempts?: number;
    // NetworkInfo
    /** Protocol parameters */
    protocolParameters?: ProtocolParameters;
    /** The current tangle time */
    tangleTime?: number;
    /** Timeout for API requests */
    apiTimeout?: IDuration;
    /** The maximum parallel API requests. */
    maxParallelApiRequests?: number;
}

/** Time duration */
export interface IDuration {
    /** Seconds. */
    secs: number;
    /** Nanoseconds. */
    nanos: number;
}
