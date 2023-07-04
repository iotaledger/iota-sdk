// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, WalletOptions, CoinType } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./exchange/1-create-account.ts 

// This example creates a new database and account.
async function run() {
    try {
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
        if (!process.env.MNEMONIC) {
            throw new Error(
                '.env MNEMONIC is undefined, see .env.example',
            );
        }

        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL],
            },
            // CoinType.IOTA can be used to access Shimmer staking rewards, but it's
            // recommended to use the Shimmer coin type to be compatible with other wallets.
            coinType: CoinType.Shimmer,
            secretManager: {
                stronghold: {
                    snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                    password: `${process.env.STRONGHOLD_PASSWORD}`,
                },
            },
        };

        const wallet = new Wallet(walletOptions);

        // Mnemonic only needs to be set the first time.
        await wallet.storeMnemonic(process.env.MNEMONIC);

        const account = await wallet.createAccount({
            alias: 'Alice'
        });
        console.log('Account created:', account);

    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
