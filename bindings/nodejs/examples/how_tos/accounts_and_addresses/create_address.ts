// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create_address.ts

// This example creates an address
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // To create an address we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const address = (await account.generateEd25519Addresses(1))[0];

        console.log(`Generated address:`, address.address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
