// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    MetadataFeature,
    SenderFeature,
    Ed25519Address,
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
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const accountOutput = await client.buildAccountOutput({
            accountId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new AddressUnlockCondition(new Ed25519Address(hexAddress)),
            ],
            features: [
                new SenderFeature(new Ed25519Address(hexAddress)),
                new MetadataFeature(utf8ToHex('hello')),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
                new MetadataFeature(utf8ToHex('hello')),
            ],
        });

        console.log(JSON.stringify(accountOutput, null, 2));
        process.exit();
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
