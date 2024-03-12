// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { InfoResponse } from '../models/api';

/** NodeInfo wrapper which contains the node info and the url from the node (useful when multiple nodes are used) */
export interface NodeInfoResponse {
    /** The node info */
    info: InfoResponse;
    /** The url of the node */
    url: string;
}
