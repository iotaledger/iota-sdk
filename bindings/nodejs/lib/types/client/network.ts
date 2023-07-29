// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { INodeInfoProtocol } from '../models/info';

/**
 * Network types.
 */
export enum Network {
    /** TODO */
    Mainnet,
    /** TODO */
    Testnet,
}

/**
 * Basic Auth or JWT.
 */
export interface IAuth {
    /** TODO */
    jwt?: string;
    /** TODO */
    basicAuthNamePwd?: [string, string];
}

/**
 * Options for the MQTT broker.
 */
export interface IMqttBrokerOptions {
    /** TODO */
    automaticDisconnect?: boolean;
    /** timeout in seconds */
    timeout?: number;
    /** TODO */
    useWs?: boolean;
    /** TODO */
    port?: number;
    /** TODO */
    maxReconnectionAttempts?: number;
}

/**
 * A node object for the client.
 */
export interface INode {
    /** TODO */
    url: string;
    /** TODO */
    auth?: IAuth;
    /** TODO */
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
