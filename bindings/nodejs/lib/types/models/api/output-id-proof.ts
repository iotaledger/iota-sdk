// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { plainToInstance } from 'class-transformer';
import { SlotIndex } from '../../block';
import { HexEncodedString } from '../../utils';

/**
 * Node types.
 */
export enum TreeNodeType {
    /** Denotes a HashableNode */
    HashableNode = 0,
    /** Denotes a LeafHash */
    LeafHash = 1,
    /** Denotes a ValueHash */
    ValueHash = 2,
}

export abstract class TreeNode {
    readonly type: TreeNodeType;

    /**
     * @param type The type of tree node.
     */
    constructor(type: TreeNodeType) {
        this.type = type;
    }

    /**
     * Parse a tree node from a plain JS JSON object.
     */
    public static parse(data: any): TreeNode {
        if (data.type == TreeNodeType.HashableNode) {
            return plainToInstance(HashableNode, data) as any as HashableNode;
        } else if (data.type == TreeNodeType.LeafHash) {
            return plainToInstance(LeafHash, data) as any as LeafHash;
        } else if (data.type == TreeNodeType.ValueHash) {
            return plainToInstance(ValueHash, data) as any as ValueHash;
        }
        throw new Error('Invalid JSON');
    }
}

/**
 * Contains the hashes of the left and right children of a node in the tree.
 */
export class HashableNode extends TreeNode {
    readonly l: TreeNode;
    readonly r: TreeNode;

    /**
     * @param l Output commitment proof of the left sub-tree.
     * @param r Output commitment proof of the right sub-tree.
     */
    constructor(l: TreeNode, r: TreeNode) {
        super(TreeNodeType.HashableNode);
        this.l = l;
        this.r = r;
    }
}

/**
 * Contains the hash of a leaf in the tree.
 */
export class LeafHash extends TreeNode {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the leaf.
     */
    constructor(hash: HexEncodedString) {
        super(TreeNodeType.LeafHash);
        this.hash = hash;
    }
}

/**
 * Contains the hash of the value for which the proof is being computed.
 */
export class ValueHash extends TreeNode {
    readonly hash: HexEncodedString;

    /**
     * @param hash The hash of the value.
     */
    constructor(hash: HexEncodedString) {
        super(TreeNodeType.LeafHash);
        this.hash = hash;
    }
}

export class OutputIdProof {
    readonly slot: SlotIndex;
    readonly outputIndex: number;
    readonly transactionCommitment: HexEncodedString;
    readonly outputCommitmentProof: TreeNode;

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
        outputCommitmentProof: TreeNode,
    ) {
        this.slot = slot;
        this.outputIndex = outputIndex;
        this.transactionCommitment = transactionCommitment;
        this.outputCommitmentProof = outputCommitmentProof;
    }
}
