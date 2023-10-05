// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SecretManager, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./secret_manager/stronghold.ts

// In this example we will store a mnemonic in the Stronghold vault and generate an address
async function run() {
    initLogger();

    try {
        for (const envVar of ['MNEMONIC', 'STRONGHOLD_PASSWORD']) {
            if (!(envVar in process.env)) {
                throw new Error(`.env ${envVar} is not defined`);
            }
        }
        const strongholdSecretManager = new SecretManager({
            stronghold: {
                password: process.env.STRONGHOLD_PASSWORD,
                snapshotPath: 'client.stronghold',
            },
        });

        // A mnemonic can be generated with `Utils.generateMnemonic()`.
        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        await strongholdSecretManager.storeMnemonic(
            process.env.MNEMONIC as string,
        );

        const address = await strongholdSecretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'rms',
        });

        console.log('First public address:', address, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
