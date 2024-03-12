// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { BasicOutput, BlockId, OutputId, TransactionId, Utils } from '../../out';
import '../customMatchers';
import { SlotCommitment } from '../../out/types/block/slot';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { ProtocolParameters } from '../../lib/types/models/api';

describe('Utils methods', () => {
    it('invalid mnemonic error', () => {
        try {
            Utils.verifyMnemonic('invalid mnemonic '.repeat(12));
            throw 'should error';
        } catch (e: any) {
            expect(e.message).toContain('NoSuchWord');
        }
    });

    it('converts hex public key to bech32 address', () => {
        const hexPublicKey =
            '0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a';

        const address = Utils.publicKeyHash(hexPublicKey);

        expect(address.pubKeyHash).toBe('0x96f9de0989e77d0e150e850a5a600e83045fa57419eaf3b20225b763d4e23813');
    });

    it('validates address', () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';

        const isAddressValid = Utils.isAddressValid(address);

        expect(isAddressValid).toBeTruthy();
    });

    it('hash output id', () => {
        const outputId =
            '0x0000000000000000000000000000000000000000000000000000000000000000000000000000';

        const accountId = Utils.computeAccountId(outputId);

        expect(accountId).toBe(
            '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345',
        );
    });

    it('compute foundry id', () => {
        const accountId =
            '0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f';
        const serialNumber = 0;
        const tokenSchemeType = 0;

        const foundryId = Utils.computeFoundryId(
            accountId,
            serialNumber,
            tokenSchemeType,
        );

        expect(foundryId).toBe(
            '0x08cf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f0000000000',
        );
    });

    it('slot commitment id', () => {
        let slotCommitment = new SlotCommitment(
            1,
            10,
            "0x20e07a0ea344707d69a08b90be7ad14eec8326cf2b8b86c8ec23720fab8dcf8ec43a30e4",
            "0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f",
            BigInt(5),
            BigInt(6000)
        );
        let id = Utils.computeSlotCommitmentId(slotCommitment);
        expect(id).toBe(
            '0x1d1470e10ed1c498c88002d57d6eaa0db38a31347e1aa5e957300a48967f0ca40a000000'
        );
    });

    it('compute slot index from block or transaction id', async () => {
        const blockId: BlockId = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";
        expect(Utils.computeSlotIndex(blockId)).toEqual(0);
        const transactionId: TransactionId = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64901000000";
        expect(Utils.computeSlotIndex(transactionId)).toEqual(1);
    });

    it('compute output index from an output id', async () => {
        const outputId: OutputId = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00";
        const outputIndex = Utils.outputIndexFromOutputId(outputId);
        expect(outputIndex).toEqual(42);
    });

    it('compute transaction id from an output id', async () => {
        const outputId: OutputId = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00";
        const transactionId = Utils.transactionIdFromOutputId(outputId);
        expect(transactionId).toEqual("0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000");
    });

    it('decayed mana', () => {
        const protocolParameters = protocol_parameters.params as unknown as ProtocolParameters;
        const output = {
            "type": 0,
            "amount": "100000",
            "mana": "4000",
            "unlockConditions": [
                {
                    "type": 0,
                    "address": {
                        "type": 0,
                        "pubKeyHash": "0xed1484f4d1f7d8c037087fed661dd92faccae1eed3c01182d6fdd6828cea144a"
                    }
                }
            ]
        } as unknown as BasicOutput;
        const creationSlot = 5
        const targetSlot = 5000000

        let decayedMana = Utils.outputManaWithDecay(
            output, creationSlot, targetSlot, protocolParameters)
        expect(decayedMana.stored).toBe(BigInt(2272));
        expect(decayedMana.potential).toBe(BigInt(2502459));

        const decayedStoredMana = Utils.manaWithDecay(
            output.mana, creationSlot, targetSlot, protocolParameters)
        expect(decayedStoredMana).toBe(BigInt(2272));

        // storage deposit doesn't generate mana
        const minimumOutputAmount = Utils.computeMinimumOutputAmount(
            output, protocolParameters.storageScoreParameters)
        const decayedPotentialMana = Utils.generateManaWithDecay(
            BigInt(output.amount) - minimumOutputAmount, creationSlot, targetSlot, protocolParameters)
        expect(decayedPotentialMana).toBe(BigInt(2502459));
    });
});
