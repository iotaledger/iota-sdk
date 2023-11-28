// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BasicBlockBody } from './basic';
import { ValidationBlockBody } from './validation';
/**
 * All of the block body types.
 */
export enum BlockBodyType {
    /// A Basic block body.
    Basic = 0,
    /// A Validation block body.
    Validation = 1,
}

export abstract class BlockBody {
    readonly type: BlockBodyType;

    /**
     * @param type The type of BlockBody.
     */
    constructor(type: BlockBodyType) {
        this.type = type;
    }

    /**
     * Checks whether the block body is a `BasicBlockBody`.
     * @returns true if it is, otherwise false
     */
    isBasic(): boolean {
        return this.type === BlockBodyType.Basic;
    }

    /**
     * Gets the block body as an actual `BasicBlockBody`.
     * NOTE: Will throw an error if the block is not a `BasicBlockBody`.
     * @returns The BasicBlockBody
     */
    asBasic(): BasicBlockBody {
        if (this.isBasic()) {
            return this as unknown as BasicBlockBody;
        } else {
            throw new Error('invalid downcast of non-BasicBlockBody');
        }
    }

    /**
     * Checks whether the block body is a `ValidationBlockBody`.
     * @returns true if it is, otherwise false
     */
    isValidation(): boolean {
        return this.type === BlockBodyType.Validation;
    }

    /**
     * Gets the block body as an actual `ValidationBlockBody`.
     * NOTE: Will throw an error if the block is not a `ValidationBlockBody`.
     * @returns The ValidationBlockBody
     */
    asValidation(): ValidationBlockBody {
        if (this.isValidation()) {
            return this as unknown as ValidationBlockBody;
        } else {
            throw new Error('invalid downcast of non-ValidationBlockBody');
        }
    }
}
