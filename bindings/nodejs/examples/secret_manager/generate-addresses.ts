// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SecretManager, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./secret_manager/generate-addresses.ts

// In this example we will create addresses from a mnemonic defined in .env
async function run() {
    initLogger();
    for (const envVar of ['MNEMONIC']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const mnemonicSecretManager = {
            mnemonic: process.env.MNEMONIC as string,
        };

        const secretManager = new SecretManager(mnemonicSecretManager);

        // Generate public address with default account index and range.
        const default_addresses = await secretManager.generateEd25519Addresses(
            {},
        );
        console.log(
            'List of generated public addresses: ',
            default_addresses,
            '\n',
        );

        // Generate public address with custom account index and range.
        const address = await secretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 4,
            },
        });
        console.log('List of generated public addresses:', address, '\n');

        // Generate internal addresses with custom account index and range.
        const internalAddresses = await secretManager.generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 4,
            },
            options: { internal: true },
        });
        console.log(
            'List of generated internal addresses:',
            internalAddresses,
            '\n',
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
