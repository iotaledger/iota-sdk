// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';
import 'dotenv/config';

import { Client } from '../../lib/client';
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

// Skip for CI
describe.skip('UTXO methods', () => {
    it('gets accounts output IDs', async () => {
        const client = await makeClient();
        const accountsOutputIds = await client.accountOutputIds(
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        );

        expect(accountsOutputIds).toBeDefined();
    });

    it('gets nfts output IDs', async () => {
        const client = await makeClient();
        const nftsOutputIds = await client.nftOutputIds(
            {
                address:
                    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
            },
        );

        expect(nftsOutputIds).toBeDefined();
    });

    it('gets foundries output IDs', async () => {
        const client = await makeClient();
        const foundriesOutputIds = await client.foundryOutputIds(
            {
                hasNativeToken: true,
            },
        );

        expect(foundriesOutputIds).toBeDefined();
    });

    // TODO: get valid IDs to test with
    it('get account/nft/foundry outputId rejects with 404 for invalid IDs', async () => {
        const client = await makeClient();
        await expect(
            client.accountOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a95',
            ),
        ).rejects.toMatch('404');

        await expect(
            client.nftOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a95',
            ),
        ).rejects.toMatch('404');

        await expect(
            client.foundryOutputId(
                '0x03119f37e7ad40608fc7ab15db49390abc233648c95e78141ff2e298f60d7a9541ff2e60d7a9',
            ),
        ).rejects.toMatch('404');
    });
});
