// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    SecretManager,
    CoinType,
    initLogger,
    SHIMMER_TESTNET_BECH32_HRP,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/secretManager/generate_addresses.js

// In this example we will create addresses from a mnemonic defined in .env
async function run() {
    initLogger();

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const mnemonicSecretManager = {
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        };

        const secretManager = new SecretManager(mnemonicSecretManager);

        // Generate public address with default account index and range.
        const default_addresses = await secretManager.generateAddresses({});
        console.log(
            'List of generated public addresses: ',
            default_addresses,
            '\n',
        );

        // Generate public address with custom account index and range.
        const address = await secretManager.generateAddresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 4,
            },
        });
        console.log('List of generated public addresses:', address, '\n');

        // Generate internal addresses with custom account index and range.
        const internalAddresses = await secretManager.generateAddresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 4,
            },
            internal: true,
        });
        console.log(
            'List of generated internal addresses:',
            internalAddresses,
            '\n',
        );

        // Generate addresses with providing all inputs, that way it can also be done offline without a node.
        const offlineGeneratedAddresses = await secretManager.generateAddresses(
            {
                coinType: CoinType.Shimmer,
                accountIndex: 0,
                range: {
                    start: 0,
                    end: 4,
                },
                internal: false,
                // Generating addresses with client.generateAddresses(secretManager, {}), will by default get the bech32_hrp (Bech32
                // human readable part) from the node info, generating it "offline" requires setting it in the generateAddressesOptions
                bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
            },
        );
        console.log(
            'List of offline generated public addresses:',
            offlineGeneratedAddresses,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
