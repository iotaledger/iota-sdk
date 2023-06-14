// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Block, MilestonePayload } from '../types';
import { Utils } from './utils';

/**
 * Compute a milestoneId from a milestone payload.
 * @param payload The milestone payload.
 * @returns The milestone id hex prefixed string.
 */
export function milestoneIdFromMilestonePayload(
    payload: MilestonePayload,
): string {
    return Utils.milestoneId(payload);
}

/**
 * Compute a blockId from a milestone payload.
 * @param protocolVersion The protocol version to use.
 * @param payload The milestone payload.
 * @returns The blockId of the block with the milestone payload.
 */
export function blockIdFromMilestonePayload(
    protocolVersion: number,
    payload: MilestonePayload,
) {
    const block: Block = {
        protocolVersion,
        parents: payload.parents,
        payload,
        nonce: '0',
    };

    return Utils.blockId(block);
}
