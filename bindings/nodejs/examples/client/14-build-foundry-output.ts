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

// In this example we will build a foundry output.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL', 'MNEMONIC']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
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
