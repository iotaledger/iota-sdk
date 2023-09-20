// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockType } from './block';
import { BasicBlock } from './basic';

export * from './block';
export * from './basic';
export * from './wrapper';

// Here because in block.ts it causes a circular dependency
export const BlockDiscriminator = {
    property: 'type',
    subTypes: [{ value: BasicBlock, name: BlockType.Basic as any }],
};
