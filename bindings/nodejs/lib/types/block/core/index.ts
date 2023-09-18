// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { BasicBlock } from './basic';
import { BlockType } from './block';
import { BlockWrapper } from './wrapper';

export * from './wrapper';
export * from './basic';
export * from './block';


export const BlockDiscriminator = {
    property: 'type',
    subTypes: [{ value: BasicBlock, name: BlockType.Basic as any }],
};

export function parseBlockWrapper(data: any): BlockWrapper {
    return plainToInstance(BlockWrapper, data) as any as BlockWrapper;
}
