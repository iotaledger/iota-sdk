// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
export interface Auth {
    /** JWT authentication parameters. */
    jwt?: string;
    /** Basic authentication parameters. */
    basicAuthNamePwd?: [string, string];
}

/**
 * Options for the MQTT broker.
 */
export interface MqttBrokerOptions {
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
export interface Node {
    /** The URL of the node. */
    url: string;
    /** The authentication parameters. */
    auth?: Auth;
    /** Whether the node is disabled or not. */
    disabled?: boolean;
    /** Whether the node is a permanode or not. */
    permanode?: boolean;
}
