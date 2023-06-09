// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IGossipHeartbeat } from './gossip-heartbeat';
import type { IGossipMetrics } from './gossip-metrics';
/**
 * Peer details.
 */
export interface IPeer {
    /**
     * The id of the peer.
     */
    id: string;
    /**
     * The addresses of the peer.
     */
    multiAddresses: string[];
    /**
     * The alias of the peer.
     */
    alias?: string;
    /**
     * The relation of the peer.
     */
    relation: string;
    /**
     * Is it connected.
     */
    connected: boolean;
    /**
     * Gossip metrics for the peer.
     */
    gossip?: {
        /**
         * The peer heartbeat.
         */
        heartbeat: IGossipHeartbeat;
        /**
         * The peer metrics.
         */
        metrics: IGossipMetrics;
    };
}
