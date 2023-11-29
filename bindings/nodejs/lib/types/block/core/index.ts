// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockBodyType } from './block-body';
import { BasicBlockBody } from './basic';
import { ValidationBlockBody } from './validation';

export * from './block';
export * from './block-body';
export * from './basic';
export * from './validation';

// Here because in block-body.ts it causes a circular dependency
export const BlockBodyDiscriminator = {
    property: 'type',
    subTypes: [
        { value: BasicBlockBody, name: BlockBodyType.Basic as any },
        { value: ValidationBlockBody, name: BlockBodyType.Validation as any },
    ],
};
