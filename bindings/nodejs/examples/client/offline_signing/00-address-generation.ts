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
// yarn run-example ./client/offline_signing/00-address-generation.ts

const ADDRESS_FILE_NAME = 'offline-signing-address.json';

// In this example we will generate an address offline which will be used later to find inputs.
async function run() {
    initLogger();
    for (const envVar of ['MNEMONIC']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    try {
        const secretManager = new SecretManager({
            mnemonic: process.env.MNEMONIC as string,
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
        console.log(
            'Address generated and saved to file: ' + ADDRESS_FILE_NAME + '',
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
