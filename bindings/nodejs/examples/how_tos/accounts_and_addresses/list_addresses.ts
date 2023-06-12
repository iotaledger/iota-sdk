// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/list_addresses.ts

// This example lists all addresses in the account
async function run() {
    initLogger();
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        const addresses = await account.addresses();

        for (const address of addresses) console.log(address.address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
