// Copyright 2021-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ProtocolParameters } from '../models/info';

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
 * A node object for the client.
 */
export interface INode {
    /** The URL of the node. */
    url: string;
    /** The authentication parameters. */
    auth?: IAuth;
    /** Whether the node is disabled or not. */
    disabled?: boolean;
    /** Whether the node is a permanode or not. */
    permanode?: boolean;
}

/**
 * Struct containing network related information
 */
export interface INetworkInfo {
    /** Protocol parameters */
    protocolParameters: ProtocolParameters;
}
