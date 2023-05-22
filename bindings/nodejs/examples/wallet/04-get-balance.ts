// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedManager } from './accounts';

// In this example we sync the account and get the balance.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/04-get-balance.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // Sync and get the balance
        let _ = await account.sync();
        // If already synced, just get the balance
        let balance = await account.getBalance();

        console.log(balance);

        console.log('Addresses:');
        let prepended = process.env.EXPLORER_URL + '/addr/';
        for (let address of await account.addresses()) {
            console.log(' - ' + prepended + address.address);
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
