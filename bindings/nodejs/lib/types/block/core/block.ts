// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountId } from '../id';
import { Signature, SignatureDiscriminator } from '../signature';
import { SlotCommitmentId, SlotIndex } from '../slot';
import { u64 } from '../../utils/type-aliases';
import { plainToInstance, Type } from 'class-transformer';
import { BlockBody, BlockBodyType } from './block-body';
import { BasicBlockBody } from './basic';
import { ValidationBlockBody } from './validation';
import { BlockBodyDiscriminator } from '.';

/**
 * Represent the object that nodes gossip around the network.
 */
class Block {
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
    readonly issuerId!: AccountId;

    @Type(() => BlockBody, {
        discriminator: BlockBodyDiscriminator,
    })
    readonly body!: BlockBody;

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
        issuerId: AccountId,
        body: BlockBody,
        signature: Signature,
    ) {
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        this.slotCommitmentId = slotCommitmentId;
        this.latestFinalizedSlot = latestFinalizedSlot;
        this.issuerId = issuerId;
        this.body = body;
        this.signature = signature;
    }

    /**
     * Checks whether the block body is a `BasicBlockBody`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.body.type === BlockBodyType.Basic;
    }

    /**
     * Gets the block body as an actual `BasicBlockBody`.
     * NOTE: Will throw an error if the block body is not a `BasicBlockBody`.
     * @returns The block
     */
    asBasic(): BasicBlockBody {
        if (this.isBasic()) {
            return this.body as unknown as BasicBlockBody;
        } else {
            throw new Error('invalid downcast of non-BasicBlockBody');
        }
    }

    /**
     * Checks whether the block body is a `ValidationBlockBody`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.body.type === BlockBodyType.Validation;
    }

    /**
     * Gets the block body as an actual `ValidationBlockBody`.
     * NOTE: Will throw an error if the block body is not a `ValidationBlockBody`.
     * @returns The block
     */
    asValidation(): ValidationBlockBody {
        if (this.isValidation()) {
            return this.body as unknown as ValidationBlockBody;
        } else {
            throw new Error('invalid downcast of non-ValidationBlockBody');
        }
    }
}

function parseBlock(data: any): Block {
    return plainToInstance(Block, data) as any as Block;
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
    readonly issuerId!: AccountId;

    @Type(() => BlockBody, {
        discriminator: BlockBodyDiscriminator,
    })
    readonly body!: BlockBody;

    constructor(
        protocolVersion: number,
        networkId: u64,
        issuingTime: u64,
        slotCommitmentId: SlotCommitmentId,
        latestFinalizedSlot: SlotIndex,
        issuerId: AccountId,
        body: BlockBody,
    ) {
        this.protocolVersion = protocolVersion;
        this.networkId = networkId;
        this.issuingTime = issuingTime;
        this.slotCommitmentId = slotCommitmentId;
        this.latestFinalizedSlot = latestFinalizedSlot;
        this.issuerId = issuerId;
        this.body = body;
    }

    /**
     * Checks whether the block body is a `BasicBlockBody`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.body.type === BlockBodyType.Basic;
    }

    /**
     * Gets the block body as an actual `BasicBlock`.
     * NOTE: Will throw an error if the block body is not a `BasicBlockBody`.
     * @returns The BasicBlockBody
     */
    asBasic(): BasicBlockBody {
        if (this.isBasic()) {
            return this.body as unknown as BasicBlockBody;
        } else {
            throw new Error('invalid downcast of non-BasicBlockBody');
        }
    }

    /**
     * Checks whether the block body is a `ValidationBlockBody`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.body.type === BlockBodyType.Validation;
    }

    /**
     * Gets the block body as an actual `ValidationBlockBody`.
     * NOTE: Will throw an error if the block body is not a `ValidationBlockBody`.
     * @returns The ValidationBlockBody
     */
    asValidation(): ValidationBlockBody {
        if (this.isValidation()) {
            return this.body as unknown as ValidationBlockBody;
        } else {
            throw new Error('invalid downcast of non-ValidationBlockBody');
        }
    }
}

function parseUnsignedBlock(data: any): UnsignedBlock {
    return plainToInstance(UnsignedBlock, data) as any as UnsignedBlock;
}

export { Block, parseBlock, UnsignedBlock, parseUnsignedBlock };
