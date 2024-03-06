// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    MetadataFeature,
    SenderFeature,
    IssuerFeature,
    AddressUnlockCondition,
    utf8ToHex,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/13-build-account-output.ts

// In this example we will build an account output.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
        const ed25519Address = Utils.parseBech32Address(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const accountOutput = await client.buildAccountOutput({
            accountId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [new AddressUnlockCondition(ed25519Address)],
            features: [
                new SenderFeature(ed25519Address),
                new MetadataFeature({ data: utf8ToHex('hello') }),
            ],
            immutableFeatures: [
                new IssuerFeature(ed25519Address),
                new MetadataFeature({ data: utf8ToHex('hello') }),
            ],
        });

        console.log(JSON.stringify(accountOutput, null, 2));
        process.exit();
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
