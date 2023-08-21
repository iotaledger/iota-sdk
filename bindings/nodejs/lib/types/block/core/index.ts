// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { BasicBlock, BasicBlockData } from './basic-block';
import { BlockType, BlockWrapper } from './block';

export * from './block';
export * from './basic-block';

export type Block = BasicBlock;

export const BlockDiscriminator = {
    property: 'type',
    subTypes: [
        { value: BlockWrapper<BasicBlockData>, name: BlockType.Basic as any },
    ],
};

export function parseBlock(data: any): Block {
    if (data.type == BlockType.Basic) {
        return plainToInstance(
            BlockWrapper<BasicBlockData>,
            data,
        ) as any as BlockWrapper<BasicBlockData>;
    }
    throw new Error('Invalid JSON');
}
