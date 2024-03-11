// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { plainToInstance } from 'class-transformer';
import { SlotIndex } from '../../../block';
import { HexEncodedString } from '../../../utils';

/**
 * OutputCommitmentProof types.
 */
export enum OutputCommitmentProofType {
    /** Denotes a HashableNode. */
    HashableNode = 0,
    /** Denotes a LeafHash. */
    LeafHash = 1,
    /** Denotes a ValueHash. */
    ValueHash = 2,
}

export abstract class OutputCommitmentProof {
    readonly type: OutputCommitmentProofType;

    /**
     * @param type The type of OutputCommitmentProof.
     */
    constructor(type: OutputCommitmentProofType) {
        this.type = type;
    }

    /**
     * Parse an OutputCommitmentProof from a plain JSON object.
     */
    public static parse(data: any): OutputCommitmentProof {
        if (data.type == OutputCommitmentProofType.HashableNode) {
            return plainToInstance(HashableNode, data) as any as HashableNode;
        } else if (data.type == OutputCommitmentProofType.LeafHash) {
            return plainToInstance(LeafHash, data) as any as LeafHash;
        } else if (data.type == OutputCommitmentProofType.ValueHash) {
            return plainToInstance(ValueHash, data) as any as ValueHash;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * Contains the hashes of the left and right children of a node in the OutputCommitmentProof tree.
 */
export class HashableNode extends OutputCommitmentProof {
    readonly l: OutputCommitmentProof;
    readonly r: OutputCommitmentProof;

    /**
     * @param l Output commitment proof of the left subtree.
     * @param r Output commitment proof of the right subtree.
     */
    constructor(l: OutputCommitmentProof, r: OutputCommitmentProof) {
        super(OutputCommitmentProofType.HashableNode);
        this.l = l;
        this.r = r;
    }
}

/**
 * Contains the hash of a leaf in the OutputCommitmentProof tree.
 */
export class LeafHash extends OutputCommitmentProof {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the leaf.
     */
    constructor(hash: HexEncodedString) {
        super(OutputCommitmentProofType.LeafHash);
        this.hash = hash;
    }
}

/**
 * Contains the hash of the value for which the OutputCommitmentProof is being computed.
 */
export class ValueHash extends OutputCommitmentProof {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the value.
     */
    constructor(hash: HexEncodedString) {
        super(OutputCommitmentProofType.ValueHash);
        this.hash = hash;
    }
}

/**
 * The proof of the output identifier.
 */
export class OutputIdProof {
    readonly slot: SlotIndex;
    readonly outputIndex: number;
    readonly transactionCommitment: HexEncodedString;
    readonly outputCommitmentProof: OutputCommitmentProof;

    /**
     * @param slot The slot index of the output.
     * @param outputIndex The index of the output within the corresponding transaction.
     * @param transactionCommitment The commitment of the transaction that created the output. Hex-encoded with 0x prefix.
     * @param outputCommitmentProof The proof of the output commitment.
     */
    constructor(
        slot: SlotIndex,
        outputIndex: number,
        transactionCommitment: HexEncodedString,
        outputCommitmentProof: OutputCommitmentProof,
    ) {
        this.slot = slot;
        this.outputIndex = outputIndex;
        this.transactionCommitment = transactionCommitment;
        this.outputCommitmentProof = outputCommitmentProof;
    }
}
