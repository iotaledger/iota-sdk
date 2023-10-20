// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, CoinType, initLogger, WalletOptions } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create-account.ts

// This example creates a new database and account.
async function run() {
    initLogger();
    for (const envVar of [
        'NODE_URL',
        'STRONGHOLD_PASSWORD',
        'STRONGHOLD_SNAPSHOT_PATH',
        'MNEMONIC',
        'WALLET_DB_PATH',
    ])
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }

    try {
        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL as string],
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
        await wallet.storeMnemonic(process.env.MNEMONIC as string);

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
