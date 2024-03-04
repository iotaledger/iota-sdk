// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { Client } from '../../lib/client';
import '../customMatchers';
import protocolParametersFixture from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';

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

        const infoByUrl = await client.getInfo(nodeInfo.url);

        expect(infoByUrl).toBeDefined();
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
        const tips = (await client.getIssuance()).strongParents;

        expect(tips.length).toBeGreaterThan(0);
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

describe('Offline client info methods', () => {
    it('provided protocol parameters', async () => {
        const protocolParameters = protocolParametersFixture.params;
        const client = await Client.create({
            protocolParameters
        });
        const params = await client.getProtocolParameters();

        expect(params).toStrictEqual(protocolParameters);
    });
})
