// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    StateControllerAddressUnlockCondition,
    MetadataFeature,
    SenderFeature,
    Ed25519Address,
    IssuerFeature,
    GovernorAddressUnlockCondition,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/13_build_alias_output.ts

// Build a basic output
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const aliasOutput = await client.buildAliasOutput({
            aliasId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            // `hello` hex encoded
            stateMetadata: '0x68656c6c6f',
            unlockConditions: [
                new StateControllerAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
                new GovernorAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
            ],
            features: [
                new SenderFeature(new Ed25519Address(hexAddress)),
                // `hello` hex encoded
                new MetadataFeature('0x68656c6c6f'),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
                // `hello` hex encoded
                new MetadataFeature('0x68656c6c6f'),
            ],
        });

        console.log(JSON.stringify(aliasOutput, null, 2));
        process.exit();
    } catch (error) {
        console.error('Error: ', error);
    }
}

run();
