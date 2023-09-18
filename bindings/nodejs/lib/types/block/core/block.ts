// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BasicBlock } from './basic';
/**
 * All of the block types.
 */
 export enum BlockType {
    /// A Basic block.
    Basic = 0,
}

export abstract class Block {
    readonly type: BlockType;

    /**
     * @param type The type of Block.
     */
    constructor(type: BlockType) {
        this.type = type;
    }

    /**
     * Checks whether the block is a `BasicBlock`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.type === BlockType.Basic;
    }

    /**
     * Gets the block as an actual `BasicBlock`.
     * NOTE: Will throw an error if the block is not a `BasicBlock`.
     * @returns The block
     */
    asBasic(): BasicBlock {
        if (this.isBasic()) {
            return this as unknown as BasicBlock;
        } else {
            throw new Error('invalid downcast of non-BasicBlock');
        }
    }
}
