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
    private type: BlockType;

    /**
     * Protocol version of the block.
     */
    private protocolVersion: number;
    /**
     * Network identifier.
     */
    private networkId: bigint;
    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    private issuingTime: bigint;
    /**
     * The identifier of the slot to which this block commits.
     */
    private slotCommitmentId: SlotCommitmentId;
    /**
     * The slot index of the latest finalized slot.
     */
    private latestFinalizedSlot: bigint;
    /**
     * The identifier of the account that issued this block.
     */
    private issuerId: IssuerId;
    /**
     * The block signature; used to validate issuance capabilities.
     */
    private signature: Ed25519Signature;

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

    /**
     * The type of block.
     */
    getType(): BlockType {
        return this.type;
    }

    /**
     * Protocol version of the block.
     */
    getProtocolVersion(): number {
        return this.protocolVersion;
    }

    /**
     * Network identifier.
     */
    getNetworkId(): bigint {
        return this.networkId;
    }

    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    getIssuingTime(): bigint {
        return this.issuingTime;
    }

    /**
     * The identifier of the slot to which this block commits.
     */
    getSlotCommitmentId(): SlotCommitmentId {
        return this.slotCommitmentId;
    }

    /**
     * The slot index of the latest finalized slot.
     */
    getLatestFinalizedSlot(): bigint {
        return this.latestFinalizedSlot;
    }

    /**
     * The identifier of the account that issued this block.
     */
    getIssuerId(): IssuerId {
        return this.issuerId;
    }

    /**
     * The block signature; used to validate issuance capabilities.
     */
    getSignature(): Ed25519Signature {
        return this.signature;
    }
}

export { BlockType, BlockWrapper };
