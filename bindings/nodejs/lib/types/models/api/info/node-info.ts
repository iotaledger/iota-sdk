// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { BaseTokenResponse } from './node-info-base-token';
import type { ProtocolParametersResponse } from './node-info-protocol';
import type { StatusResponse } from './node-info-status';
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
    status: StatusResponse;
    /**
     * The protocol parameters.
     */
    protocolParameters: ProtocolParametersResponse[];
    /**
     * The base token info of the node.
     */
    baseToken: BaseTokenResponse;
}
