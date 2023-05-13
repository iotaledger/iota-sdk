// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// node ./dist/how_tos/accounts_and_addresses/create_address.js

// This example syncs the account and prints the balance
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        // To create an address we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const address = await account.generateAddresses(1);

        console.log(
            `Generated address:`,
            address[0].address,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
