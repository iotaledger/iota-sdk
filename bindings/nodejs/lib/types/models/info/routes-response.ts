// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Response from the /routes endpoint.
 */
export interface IRoutesResponse {
    /**
     * The routes the node exposes.
     */
    routes: string[];
}
