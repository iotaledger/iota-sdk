// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IssuerId, SlotCommitmentId } from '../id';
import { Ed25519Signature } from '../signature';

/**
 * All of the block types.
 */
enum BlockType {
    /// A Basic block.
    Basic = 0,
    /// A Verification block.
    Verification = 1,
}

abstract class BlockWrapper {
    /**
     * The type of block
     */
    readonly type: BlockType;

    /**
     * Protocol version of the block.
     */
    readonly protocolVersion: number;
    /**
     * Network identifier.
     */
    readonly networkId: bigint;
    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    readonly issuingTime: bigint;
    /**
     * The identifier of the slot to which this block commits.
     */
    readonly slotCommitmentId: SlotCommitmentId;
    /**
     * The slot index of the latest finalized slot.
     */
    readonly latestFinalizedSlot: bigint;
    /**
     * The identifier of the account that issued this block.
     */
    readonly issuerId: IssuerId;
    /**
     * The block signature; used to validate issuance capabilities.
     */
    readonly signature: Ed25519Signature;

    constructor(
        type: BlockType,
        protocolVersion: number,
        networkId: bigint,
        issuingTime: bigint,
        slotCommitmentId: SlotCommitmentId,
        latestFinalizedSlot: bigint,
        issuerId: IssuerId,
        signature: Ed25519Signature,
    ) {
        this.type = type;
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        this.slotCommitmentId = slotCommitmentId;
        this.latestFinalizedSlot = latestFinalizedSlot;
        this.issuerId = issuerId;
        this.signature = signature;
    }
}

export { BlockType, BlockWrapper };
