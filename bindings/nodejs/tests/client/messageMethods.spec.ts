// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import {
    CoinType,
    Client,
    utf8ToHex,
    TaggedDataPayload,
    SecretManager,
} from '../../';
import '../customMatchers';

async function makeClient(): Promise<Client> {
    return await Client.create({
        nodes: [
            {
                url: process.env.NODE_URL || 'http://localhost:8050',
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
describe.skip('Block methods', () => {
    it('sends a block raw', async () => {
        const client = await makeClient();
        const unsignedBlock = await client.buildBasicBlock(
            issuerId,
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );
        const block = await secretManager.signBlock(unsignedBlock, chain);

        const blockId = await client.postBlockRaw(block);

        expect(blockId).toBeValidBlockId();
    });

    it('finds blocks by block IDs', async () => {
        const client = await makeClient();
        const blockIds = await client.getTips();
        const blocks = await client.findBlocks(blockIds);

        expect(blocks.length).toBe(blockIds.length);
    });

    it('gets block as raw bytes', async () => {
        const client = await makeClient();
        const tips = await client.getTips();

        const blockRaw = await client.getBlockRaw(tips[0]);

        expect(blockRaw).toBeDefined();
    });
});
