// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BasicBlock } from './basic';
/**
 * All of the block types.
 */
export enum BlockType {
    /// A Basic block.
    Basic = 0,
    /// A Validation block.
    Validation = 1,
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

    /**
     * Checks whether the block is a `ValidationBlock`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.type === BlockType.Validation;
    }

    /**
     * Gets the block as an actual `ValidationBlock`.
     * NOTE: Will throw an error if the block is not a `ValidationBlock`.
     * @returns The block
     */
    asValidation(): ValidationBlock {
        if (this.isBasic()) {
            return this as unknown as ValidationBlock;
        } else {
            throw new Error('invalid downcast of non-ValidationBlock');
        }
    }
}
