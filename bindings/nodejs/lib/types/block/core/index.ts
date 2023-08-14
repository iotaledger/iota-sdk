// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { BlockType, Block } from './block';
import { BasicBlock } from './basic-block';

export * from './block';
export * from './basic-block';

export const BlockDiscriminator = {
    property: 'type',
    subTypes: [{ value: BasicBlock, name: BlockType.Basic as any }],
};

export function parseBlock(data: any): Block {
    if (data.type == BlockType.Basic) {
        return plainToInstance(BasicBlock, data) as any as BasicBlock;
    }
    throw new Error('Invalid JSON');
}
