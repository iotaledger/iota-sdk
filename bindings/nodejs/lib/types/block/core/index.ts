// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { BlockType, BlockWrapper } from './block_wrapper';
import { BasicBlock } from './basic_block';

export * from './block_wrapper';
export * from './basic_block';

export const BlockDiscriminator = {
    property: 'type',
    subTypes: [{ value: BasicBlock, name: BlockType.Basic as any }],
};

export function parseBlock(data: any): BlockWrapper {
    if (data.type == BlockType.Basic) {
        return plainToInstance(BasicBlock, data) as any as BasicBlock;
    }
    throw new Error('Invalid JSON');
}
