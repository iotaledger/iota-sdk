// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This example creates a new database and account.
// Run with command:
// yarn run-example ./exchange/1-create-account.ts

import { Wallet, WalletOptions, CoinType } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

async function run() {
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'NODE_URL',
            'STRONGHOLD_SNAPSHOT_PATH',
            'STRONGHOLD_PASSWORD',
            'MNEMONIC',
        ])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const walletOptions: WalletOptions = {
            storagePath: process.env.WALLET_DB_PATH,
            clientOptions: {
                nodes: [process.env.NODE_URL as string],
            },
            coinType: CoinType.IOTA,
            secretManager: {
                stronghold: {
                    snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                    password: process.env.STRONGHOLD_PASSWORD,
                },
            },
        };

        const wallet = new Wallet(walletOptions);

        // Mnemonic only needs to be set the first time.
        await wallet.storeMnemonic(process.env.MNEMONIC as string);

        const account = await wallet.createAccount({
            alias: 'Alice',
        });

        // Set syncOnlyMostBasicOutputs to true if not interested in outputs that are timelocked,
        // have a storage deposit return, expiration or are nft/alias/foundry outputs.
        account.setDefaultSyncOptions({ syncOnlyMostBasicOutputs: true });

        console.log(account);
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
