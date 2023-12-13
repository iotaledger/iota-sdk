// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
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
describe.skip('Client info methods', () => {
    it('gets a node candidate from the synced node pool', async () => {
        const client = await makeClient();
        const nodeInfo = await client.getNode();

        expect(nodeInfo.disabled).not.toBeTruthy();
    });

    it('gets info about node by url', async () => {
        const client = await makeClient();
        const nodeInfo = await client.getNode();

        const nodeInfoByUrl = await client.getNodeInfo(nodeInfo.url);

        expect(nodeInfoByUrl).toBeDefined();
    });

    it('gets health of node with input url', async () => {
        const client = await makeClient();
        const nodeInfo = await client.getNode();

        const nodeHealth = await client.getHealth(nodeInfo.url);

        expect(nodeHealth).toBeTruthy();
    });

    it('gets the unhealty nodes', async () => {
        const client = await makeClient();
        const unhealthyNodes = await client.unhealthyNodes();

        expect(unhealthyNodes).toBeDefined();
    });

    it('gets tips', async () => {
        const client = await makeClient();
        const tips = await client.getTips();

        expect(tips.length).toBeGreaterThan(0);
    });

    it('gets peers', async () => {
        const client = await makeClient();
        await expect(client.getPeers()).rejects.toMatch(
            'missing or malformed jwt',
        );
    });

    it('gets networkInfo', async () => {
        const client = await makeClient();
        const networkInfo = await client.getNetworkInfo();

        expect(networkInfo.protocolParameters.bech32Hrp).toBe('rms');
    });

    it('gets networkId', async () => {
        const client = await makeClient();
        const networkId = await client.getNetworkId();

        expect(networkId).toBeDefined();
    });

    it('gets bech32Hrp', async () => {
        const client = await makeClient();
        const bech32Hrp = await client.getBech32Hrp();

        expect(bech32Hrp).toBeDefined();
    });
});
