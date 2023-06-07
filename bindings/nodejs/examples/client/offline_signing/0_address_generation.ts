// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinType,
    initLogger,
    SecretManager,
    SHIMMER_TESTNET_BECH32_HRP,
} from '@iota/sdk';
import { writeFile } from 'fs/promises';

require('dotenv').config({ path: '.env' });

// From examples directory, run with:
// yarn run-example ./client/offline_signing/0_address_generation.ts

const ADDRESS_FILE_NAME = 'offline_signing_address.json';

// In this example we will generate an address offline which will be used later to find inputs
async function run() {
    initLogger();

    try {
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }

        const secretManager = new SecretManager({
            mnemonic: process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        });

        // Generates an address offline.
        const offlineGeneratedAddress =
            await secretManager.generateEd25519Addresses({
                coinType: CoinType.Shimmer,
                range: {
                    start: 0,
                    end: 1,
                },
                bech32Hrp: SHIMMER_TESTNET_BECH32_HRP,
            });

        await writeFile(
            ADDRESS_FILE_NAME,
            JSON.stringify(offlineGeneratedAddress),
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
