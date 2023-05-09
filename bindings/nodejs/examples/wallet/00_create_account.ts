// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, CoinType, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/wallet/00_create_account.js

// This example creates a new database and account
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }
    if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
        throw new Error(
            '.env NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1 is undefined, see .env.example',
        );
    }
    try {
        const walletOptions = {
            storagePath: './alice-database',
            clientOptions: {
                nodes: [process.env.NODE_URL],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: `./wallet.stronghold`,
                    password: `${process.env.STRONGHOLD_PASSWORD}`,
                },
            },
        };

        const wallet = new Wallet(walletOptions);

        // A mnemonic can be generated with `Utils.generateMnemonic()`.
        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        await wallet.storeMnemonic(
            process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        );

        const account = await wallet.createAccount({
            alias: 'Alice',
        });
        console.log('Account created:', account.getMetadata());
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
