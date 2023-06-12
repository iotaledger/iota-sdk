// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, CoinType, initLogger, WalletOptions } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create_account.ts

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
    if (!process.env.STRONGHOLD_SNAPSHOT_PATH) {
        throw new Error(
            '.env STRONGHOLD_SNAPSHOT_PATH is undefined, see .env.example',
        );
    }
    if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
        throw new Error(
            '.env NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1 is undefined, see .env.example',
        );
    }
    if (!process.env.WALLET_DB_PATH) {
        throw new Error('.env WALLET_DB_PATH is undefined, see .env.example');
    }
    try {
        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL],
            },
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                    password: process.env.STRONGHOLD_PASSWORD,
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

        // Create a new account
        const account = await wallet.createAccount({
            alias: 'Alice',
        });
        console.log('Generated new account:', account.getMetadata().alias);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
