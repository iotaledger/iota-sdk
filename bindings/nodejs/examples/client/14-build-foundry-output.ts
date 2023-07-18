// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    ImmutableAliasAddressUnlockCondition,
    AliasAddress,
    SimpleTokenScheme,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/14-build-foundry-output.ts

// Build a foundry output
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    try {
        if (!process.env.MNEMONIC) {
            throw new Error('.env MNEMONIC is undefined, see .env.example');
        }

        const aliasId =
            '0xff311f59790ccb85343a36fbac2f06d233734794404142b308c13f2c616935b5';

        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 1,
            tokenScheme: new SimpleTokenScheme(
                BigInt(10),
                BigInt(0),
                BigInt(10),
            ),
            amount: BigInt(1000000),
            unlockConditions: [
                new ImmutableAliasAddressUnlockCondition(
                    new AliasAddress(aliasId),
                ),
            ],
        });

        console.log(foundryOutput);
        process.exit();
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
