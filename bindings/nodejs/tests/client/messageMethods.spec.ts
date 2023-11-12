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

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
        },
    ],
});

const secretManager = new SecretManager({
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
        const unsignedBlock = await client.buildBasicBlock(
            issuerId,
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );
        const signedBlock = await secretManager.signBlock(unsignedBlock, chain);

        const blockId = await client.postBlockRaw(signedBlock);

        expect(blockId).toBeValidBlockId();
    });

    it('finds blocks by block IDs', async () => {
        const blockIds = await client.getTips();
        const blocks = await client.findBlocks(blockIds);

        expect(blocks.length).toBe(blockIds.length);
    });

    it('gets block as raw bytes', async () => {
        const tips = await client.getTips();

        const blockRaw = await client.getBlockRaw(tips[0]);

        expect(blockRaw).toBeDefined();
    });
});
