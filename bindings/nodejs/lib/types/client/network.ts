// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INodeInfoProtocol } from '../models/info';

/**
 * Network types.
 */
export enum Network {
    Mainnet,
    Testnet,
}

/**
 * Basic Auth or JWT.
 */
export interface IAuth {
    jwt?: string;
    basicAuthNamePwd?: [string, string];
}

/**
 * Options for the MQTT broker.
 */
export interface IMqttBrokerOptions {
    automaticDisconnect?: boolean;
    /** timeout in seconds */
    timeout?: number;
    useWs?: boolean;
    port?: number;
    maxReconnectionAttempts?: number;
}

/**
 * A node object for the client.
 */
export interface INode {
    url: string;
    auth?: IAuth;
    disabled?: boolean;
}

/**
 * Struct containing network and PoW related information
 */
export interface INetworkInfo {
    /** Protocol parameters */
    protocolParameters: INodeInfoProtocol;
}
