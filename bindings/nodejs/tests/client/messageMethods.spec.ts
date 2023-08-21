// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { Client, utf8ToHex, TaggedDataPayload } from '../../';
import '../customMatchers';

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
        },
    ],
});

// Skip for CI
describe.skip('Block methods', () => {
    it('sends a block raw', async () => {
        const blockIdAndBlock = await client.postBlockPayload(new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')));

        const blockId = await client.postBlockRaw(blockIdAndBlock[1]);

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
