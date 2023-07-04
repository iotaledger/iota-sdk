// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountManager, CoinType } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/check-balance.ts

// This example creates a new database and account.
async function run() {
    try {
        const accountManagerOptions = {
            storagePath: './alice-database',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            // CoinType.IOTA can be used to access Shimmer staking rewards, but it's
            // recommended to use the Shimmer coin type to be compatible with other wallets.
            coinType: CoinType.Shimmer,
            secretManager: {
                Stronghold: {
                    snapshotPath: `./wallet.stronghold`,
                    password: `${process.env.STRONGHOLD_PASSWORD}`,
                },
            },
        };

        const manager = new AccountManager(accountManagerOptions);

        // Mnemonic only needs to be set the first time
        await manager.storeMnemonic(process.env.MNEMONIC);

        const account = await manager.createAccount({
            alias: 'Alice'
        });
        console.log('Account created:', account);

    } catch (error) {
        console.log('Error: ', error);
    }
}

run().then(() => process.exit());
