// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { BasicBlock, BasicBlockData } from './basic-block';
import { BlockType, BlockWrapper } from './block';
import { ValidationBlock, ValidationBlockData } from './validation-block';

export * from './block';
export * from './basic-block';
export * from './validation-block';

export type Block = BasicBlock | ValidationBlock;

export const BlockDiscriminator = {
    property: 'type',
    subTypes: [
        { value: BlockWrapper<BasicBlockData>, name: BlockType.Basic as any },
        { value: BlockWrapper<ValidationBlockData>, name: BlockType.Validation as any }
    ],
};

export function parseBlock(data: any): Block {
    if (data.type == BlockType.Basic) {
        return plainToInstance(BlockWrapper<BasicBlockData>, data) as any as BasicBlock;
    } else if (data.type == BlockType.Validation) {
        return plainToInstance(BlockWrapper<ValidationBlockData>, data) as any as ValidationBlock;
    }
    throw new Error('Invalid JSON');
}
