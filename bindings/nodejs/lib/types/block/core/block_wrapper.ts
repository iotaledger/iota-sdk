// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
    type: BlockType;

    /**
     * Protocol version of the block.
     */
    protocolVersion: number;
    /**
     * Network identifier.
     */
    networkId: BigInt;
    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    issuingTime: BigInt;
    /**
     * The identifier of the slot to which this block commits.
     */
    //slotCommitmentId: SlotCommitmentId;
    /**
     * The slot index of the latest finalized slot.
     */
    //latestFinalizedSlot: SlotIndex;
    /**
     * The identifier of the account that issued this block.
     */
    //issuerId: IssuerId;
    /**
     * The block signature; used to validate issuance capabilities.
     */
    signature: Ed25519Signature;

    constructor(
        type: BlockType,
        protocolVersion: number,
        networkId: BigInt,
        issuingTime: BigInt,
        //slotCommitmentId: SlotCommitmentId,
        //latestFinalizedSlot: SlotIndex,
        //issuerId: IssuerId,
        signature: Ed25519Signature,
    ) {
        this.type = type;
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        //this.slotCommitmentId = slotCommitmentId;
        //this.latestFinalizedSlot = latestFinalizedSlot;
        //this.issuerId = issuerId;
        this.signature = signature;
    }

    /**
     * The type of block.
     */
    getType(): BlockType {
        return this.type;
    }
}

export { BlockType, BlockWrapper };
