// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IssuerId } from '../id';
import { Signature, SignatureDiscriminator } from '../signature';
import { SlotCommitmentId, SlotIndex } from '../slot';
import { u64 } from '../../utils/type-aliases';
import { plainToInstance, Type } from 'class-transformer';
import { Block, BlockType } from './block';
import { BasicBlock } from './basic';
import { ValidationBlock } from './validation';
import { BlockDiscriminator } from '.';

/**
 * Represent the object that nodes gossip around the network.
 */
class SignedBlock {
    /**
     * Protocol version of the block.
     */
    readonly protocolVersion!: number;
    /**
     * Network identifier.
     */
    readonly networkId!: u64;
    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    readonly issuingTime!: u64;
    /**
     * The identifier of the slot to which this block commits.
     */
    readonly slotCommitmentId!: SlotCommitmentId;
    /**
     * The slot index of the latest finalized slot.
     */
    readonly latestFinalizedSlot!: SlotIndex;
    /**
     * The identifier of the account that issued this block.
     */
    readonly issuerId!: IssuerId;

    @Type(() => Block, {
        discriminator: BlockDiscriminator,
    })
    readonly block!: Block;

    /**
     * The block signature; used to validate issuance capabilities.
     */
    @Type(() => Signature, {
        discriminator: SignatureDiscriminator,
    })
    readonly signature!: Signature;

    constructor(
        protocolVersion: number,
        networkId: u64,
        issuingTime: u64,
        slotCommitmentId: SlotCommitmentId,
        latestFinalizedSlot: SlotIndex,
        issuerId: IssuerId,
        block: Block,
        signature: Signature,
    ) {
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        this.slotCommitmentId = slotCommitmentId;
        this.latestFinalizedSlot = latestFinalizedSlot;
        this.issuerId = issuerId;
        this.block = block;
        this.signature = signature;
    }

    /**
     * Checks whether the block is a `BasicBlock`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.block.type === BlockType.Basic;
    }

    /**
     * Gets the block as an actual `BasicBlock`.
     * NOTE: Will throw an error if the block is not a `BasicBlock`.
     * @returns The block
     */
    asBasic(): BasicBlock {
        if (this.isBasic()) {
            return this.block as unknown as BasicBlock;
        } else {
            throw new Error('invalid downcast of non-BasicBlock');
        }
    }

    /**
     * Checks whether the block is a `ValidationBlock`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.block.type === BlockType.Validation;
    }

    /**
     * Gets the block as an actual `ValidationBlock`.
     * NOTE: Will throw an error if the block is not a `ValidationBlock`.
     * @returns The block
     */
    asValidation(): ValidationBlock {
        if (this.isValidation()) {
            return this.block as unknown as ValidationBlock;
        } else {
            throw new Error('invalid downcast of non-ValidationBlock');
        }
    }
}

function parseSignedBlock(data: any): SignedBlock {
    return plainToInstance(SignedBlock, data) as any as SignedBlock;
}

/**
 * Represent the object that nodes gossip around the network.
 */
class UnsignedBlock {
    /**
     * Protocol version of the block.
     */
    readonly protocolVersion!: number;
    /**
     * Network identifier.
     */
    readonly networkId!: u64;
    /**
     * The time at which the block was issued. It is a Unix timestamp in nanoseconds.
     */
    readonly issuingTime!: u64;
    /**
     * The identifier of the slot to which this block commits.
     */
    readonly slotCommitmentId!: SlotCommitmentId;
    /**
     * The slot index of the latest finalized slot.
     */
    readonly latestFinalizedSlot!: SlotIndex;
    /**
     * The identifier of the account that issued this block.
     */
    readonly issuerId!: IssuerId;

    @Type(() => Block, {
        discriminator: BlockDiscriminator,
    })
    readonly block!: Block;

    constructor(
        protocolVersion: number,
        networkId: u64,
        issuingTime: u64,
        slotCommitmentId: SlotCommitmentId,
        latestFinalizedSlot: SlotIndex,
        issuerId: IssuerId,
        block: Block,
    ) {
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        this.slotCommitmentId = slotCommitmentId;
        this.latestFinalizedSlot = latestFinalizedSlot;
        this.issuerId = issuerId;
        this.block = block;
    }

    /**
     * Checks whether the block is a `BasicBlock`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.block.type === BlockType.Basic;
    }

    /**
     * Gets the block as an actual `BasicBlock`.
     * NOTE: Will throw an error if the block is not a `BasicBlock`.
     * @returns The block
     */
    asBasic(): BasicBlock {
        if (this.isBasic()) {
            return this.block as unknown as BasicBlock;
        } else {
            throw new Error('invalid downcast of non-BasicBlock');
        }
    }

    /**
     * Checks whether the block is a `ValidationBlock`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.block.type === BlockType.Validation;
    }

    /**
     * Gets the block as an actual `ValidationBlock`.
     * NOTE: Will throw an error if the block is not a `ValidationBlock`.
     * @returns The block
     */
    asValidation(): ValidationBlock {
        if (this.isValidation()) {
            return this.block as unknown as ValidationBlock;
        } else {
            throw new Error('invalid downcast of non-ValidationBlock');
        }
    }
}

function parseUnsignedBlock(data: any): UnsignedBlock {
    return plainToInstance(UnsignedBlock, data) as any as UnsignedBlock;
}

export { SignedBlock, parseSignedBlock, UnsignedBlock, parseUnsignedBlock };
