// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list_accounts.ts

// This example lists all accounts in the wallet
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const accounts = await wallet.getAccounts();

        for (const account of accounts)
            console.log(account.getMetadata().alias);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
