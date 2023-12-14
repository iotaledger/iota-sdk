// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { expect, describe, it } from '@jest/globals';
import {
    Client,
    utf8ToHex,
    Utils,
    OutputResponse,
    SecretManager,
    TaggedDataPayload,
    CommonOutput,
    CoinType,
    Ed25519Address,
} from '../../';
import '../customMatchers';
import 'dotenv/config';
import * as addressOutputs from '../fixtures/addressOutputs.json';
import { AddressUnlockCondition } from '../../lib';

async function makeClient(): Promise<Client> {
    return await Client.create({
        nodes: [
            {
                url: 'http://localhost:8050',
            },
        ],
    });
}

const secretManager = SecretManager.create({
    mnemonic:
        'endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river',
});

const issuerId =
    '0x0000000000000000000000000000000000000000000000000000000000000000';

const chain = {
    coinType: CoinType.IOTA,
    account: 0,
    change: 0,
    addressIndex: 0,
};

// Skip for CI
describe.skip('Main examples', () => {
    it('gets info about the node', async () => {
        const client = await makeClient();
        const info = await client.getInfo();

        expect(
            info.nodeInfo.protocolParameters[0].parameters.bech32Hrp,
        ).toBe('rms');
    });

    it('generates a mnemonic', async () => {
        const mnemonic = Utils.generateMnemonic();

        expect(mnemonic).toBeDefined();
    });

    // TODO
    // it('generates addresses', async () => {
    //     const addresses = await SecretManager.create(
    //         secretManager,
    //     ).generateEd25519Addresses({
    //         accountIndex: 0,
    //         range: {
    //             start: 0,
    //             end: 5,
    //         },
    //         bech32Hrp: 'rms',
    //     });

    //     expect(addresses.length).toBe(5);

    //     addresses.forEach((address) => {
    //         expect(address).toBeValidAddress();
    //     });
    // });

    it('gets address outputs', async () => {
        const client = await makeClient();
        const outputIdsResponse = await client.basicOutputIds({
            address: 'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            hasExpiration: false,
            hasTimelock: false,
            hasStorageDepositReturn: false
        });

        outputIdsResponse.items.forEach((id) => expect(id).toBeValidOutputId());

        const addressOutputs = await client.getOutputs(outputIdsResponse.items);

        expect(addressOutputs).toBeDefined();

        addressOutputs.forEach((output) => {
            expect(output.metadata.blockId).toBeValidBlockId();
        });
    });

    it('gets the output of a known output ID', async () => {
        const client = await makeClient();
        const output = await client.getOutput(
            '0xc1d95ac9c8c0237c6929faf427556c3562055a7155c6d336ee7891691d5525c90100',
        );

        expect(output.metadata.blockId).toBeValidBlockId();
    });

    it('gets the balance of an address', async () => {
        // Generate the first address
        const addresses = await secretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
        });
        expect(addresses[0]).toBeValidAddress();

        const client = await makeClient();
        // Get output ids of outputs that can be controlled by this address without further unlock constraints
        const outputIdsResponse = await client.basicOutputIds({
            address: addresses[0],
            hasExpiration: false,
            hasTimelock: false,
            hasStorageDepositReturn: false,
        });
        outputIdsResponse.items.forEach((id) => expect(id).toBeValidOutputId());

        // Get outputs by their IDs
        const addressOutputs = await client.getOutputs(outputIdsResponse.items);
        expect(addressOutputs).toBeDefined();
    });

    it('calculates the balance of an address', () => {
        const testOutputs = addressOutputs as unknown as OutputResponse[];

        // Calculate the total amount and native tokens
        let totalAmount = 0;
        const totalNativeTokens: { [id: string]: number } = {};
        for (const outputResponse of testOutputs) {
            const output = outputResponse['output'];
            if (output instanceof CommonOutput) {
                const token = (output as CommonOutput).getNativeToken();
                if (token) {
                    totalNativeTokens[token.id] = (totalNativeTokens[token.id] || 0) + Number(token.amount);
                }
            }

            totalAmount += Number(output.getAmount());
        }

        expect(totalAmount).toBe(1960954000);
        expect(Object.keys(totalNativeTokens).length).toBe(2);
        expect(
            Object.values(totalNativeTokens).reduce(
                (acc: number, val: number) => acc + val,
            ),
        ).toBe(200);
    });

    // TODO: have a way in the bindings to send an empty block https://github.com/iotaledger/iota-sdk/issues/647
    // it('sends a block', async () => {
    //     const blockIdAndBlock = await client.buildAndPostBlock();

    //     expect(blockIdAndBlock[0]).toBeValidBlockId();
    // });

    it('gets block data', async () => {
        const client = await makeClient();
        const tips = await client.getTips();
        const params = await client.getProtocolParameters();

        const blockData = await client.getBlock(tips[0]);
        const blockId = Utils.blockId(blockData, params);
        expect(tips[0]).toStrictEqual(blockId);

        const blockMetadata = await client.getBlockMetadata(tips[0]);
        expect(blockMetadata.blockId).toBeValidBlockId();
    });

    it('sends a block with a tagged data payload', async () => {
        const client = await makeClient();
        const unsignedBlock = await client.buildBasicBlock(
            issuerId,
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );
        const block = await secretManager.signBlock(unsignedBlock, chain);
        const blockId = await client.postBlock(block);

        const fetchedBlock = await client.getBlock(blockId);

        expect(fetchedBlock.body.asBasic().payload).toStrictEqual(
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );
    });

    it('sends a transaction', async () => {
        const client = await makeClient();
        const addresses = await secretManager.generateEd25519Addresses({
            range: {
                start: 1,
                end: 2,
            },
        });

        const basicOutput = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(addresses[0]),
                ),
            ],
        });
        
        //let payload = await secretManager.signTransaction(prepared);
        const unsignedBlock = await client.buildBasicBlock("", undefined);
        const signedBlock = await secretManager.signBlock(unsignedBlock, chain);
        const blockId = await client.postBlock(signedBlock);

        expect(blockId).toBeValidBlockId();
    });

    it('destroy', async () => {
        const client = await makeClient();
        await client.destroy();

        try {
            const _info = await client.getInfo();
            throw 'Should return an error because the client was destroyed';
        } catch (err: any) {
            expect(err.message).toEqual('Client was destroyed');
        }
    })
});
