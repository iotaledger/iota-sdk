// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INodeInfoProtocol } from '../models/info';

/**
 * Network types.
 */
export enum Network {
    /** The mainnet. */
    Mainnet,
    /** The testnet */
    Testnet,
}

/**
 * Basic Auth or JWT.
 */
export interface IAuth {
    /** JWT authentication parameters. */
    jwt?: string;
    /** Basic authentication parameters. */
    basicAuthNamePwd?: [string, string];
}

/**
 * Options for the MQTT broker.
 */
export interface IMqttBrokerOptions {
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
}

/**
 * A node object for the client.
 */
export interface INode {
    /** The URL of the node. */
    url: string;
    /** The authentication parameters. */
    auth?: IAuth;
    /** Whether the node is disabled or not. */
    disabled?: boolean;
}

/**
 * Struct containing network and PoW related information
 */
export interface INetworkInfo {
    /** Protocol parameters */
    protocolParameters: INodeInfoProtocol;
    /** Minimum proof of work score*/
    minPowScore: number;
    /** Local proof of work */
    localPow: boolean;
    /** Fallback to local proof of work if the node doesn't support remote Pow */
    fallbackToLocalPow: boolean;
    /** Tips request interval during PoW in seconds */
    tipsInterval: number;
}
