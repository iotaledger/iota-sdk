// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { AddressUnlockCondition, AliasAddress, Client, SecretManager, Ed25519Address, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition, SimpleTokenScheme, StateControllerAddressUnlockCondition, Utils } from '../../';
import '../customMatchers';

const client = new Client({
    nodes: [
        {
            url: process.env.NODE_URL || 'http://localhost:14265',
        },
    ],
    localPow: true,
});

const secretManager = {
    mnemonic:
        'endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river',
};

// Skip for CI
describe.skip('Output builder methods', () => {
    it('builds a basic output', async () => {
        const addresses = await new SecretManager(secretManager).generateEd25519Addresses({
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = Utils.bech32ToHex(addresses[0]);

        // most simple basic output
        const basicOutput = await client.buildBasicOutput({
            amount: BigInt(1000000),
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
            ],
        });

        expect(basicOutput).toBeDefined();
    });

    it('builds an alias output', async () => {
        const addresses = await new SecretManager(secretManager).generateEd25519Addresses({
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = Utils.bech32ToHex(addresses[0]);

        const aliasId =
            '0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa';
        // most simple alias output
        const aliasOutput = await client.buildAliasOutput({
            aliasId,
            unlockConditions: [
                new StateControllerAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
                new GovernorAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
            ],
        });

        expect(aliasOutput).toBeDefined();
    });

    it('builds a foundry output', async () => {
        const aliasId =
            '0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa';

        // most simple foundry output
        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 1,
            nativeTokens: [
                {
                    id: '0x081e6439529b020328c08224b43172f282cb16649d50c891fa156365323667e47a0100000000',
                    amount: BigInt(50),
                },
            ],
            // 10 hex encoded
            tokenScheme: new SimpleTokenScheme(BigInt(10), BigInt(0), BigInt(10)),
            unlockConditions: [
                new ImmutableAliasAddressUnlockCondition(
                    new AliasAddress(aliasId),
                ),
            ],
        });

        expect(foundryOutput).toBeDefined();
    });

    it('builds an nft output', async () => {
        const addresses = await new SecretManager(secretManager).generateEd25519Addresses({
            range: {
                start: 0,
                end: 1,
            },
        });

        const hexAddress = Utils.bech32ToHex(addresses[0]);

        // most simple nft output
        const nftOutput = await client.buildNftOutput({
            nftId: '0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3',
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
            ],
        });

        expect(nftOutput).toBeDefined();
    });
});
