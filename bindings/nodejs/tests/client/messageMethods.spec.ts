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
    localPow: true,
});

// Skip for CI
describe.skip('Block methods', () => {
    it('sends a block raw', async () => {
        const blockIdAndBlock = await client.postBlockPayload(new TaggedDataPayload( utf8ToHex('Hello'), utf8ToHex('Tangle')));

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

    it('promotes a block', async () => {
        const tips = await client.getTips();

        // Promote a block without checking if it should be promoted
        const promoteUnchecked = await client.promoteUnchecked(
            tips[0],
        );

        expect(promoteUnchecked[1].parents).toContain(tips[0]);

        // Returns expected error: no need to promote or reattach.
        await expect(client.promote(tips[0])).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    it('reattaches a block', async () => {
        const tips = await client.getTips();

        // Reattach a block without checking if it should be reattached
        const reattachUnchecked = await client.reattachUnchecked(
            tips[0],
        );

        expect(reattachUnchecked[0]).toBeValidBlockId();
        expect(reattachUnchecked[1]).toBeDefined();

        // Returns expected error: no need to promote or reattach.
        await expect(client.reattach(tips[0])).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });

    // Skip by default, retryUntilIncluded can be slow
    it.skip('retries a block', async () => {
        const tips = await client.getTips();

        // Retries (promotes or reattaches) a block for provided block id until it's included
        // (referenced by a milestone). Default interval is 5 seconds and max attempts is 40.
        const retryUntilIncluded = await client.retryUntilIncluded(
            tips[0],
            2,
            5,
        );
        //Returns the included block at first position and additional reattached blocks
        expect(retryUntilIncluded[0][0]).toBe(tips[0]);

        // Returns expected error: no need to promote or reattach.
        await expect(client.retry(tips[0])).rejects.toMatch(
            'NoNeedPromoteOrReattach',
        );
    });
});
