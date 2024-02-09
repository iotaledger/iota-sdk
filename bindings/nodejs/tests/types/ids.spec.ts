// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { expect, describe, it } from '@jest/globals';
import { BlockId, OutputId, TransactionId } from '../../';

describe('ID tests', () => {

    it('get slot index', async () => {
        const blockId = new BlockId("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000")
        expect(blockId.slotIndex()).toEqual(0);
        const transactionId = new TransactionId("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64901000000")
        expect(transactionId.slotIndex()).toEqual(1);
    });

    it('get output index', async () => {
        const outputId = new OutputId("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00")
        expect(outputId.transactionId().slotIndex()).toEqual(0);
        expect(outputId.outputIndex()).toEqual(42);
    });
});
