// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger, Utils } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/create-address.ts

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

        const account = await wallet.getAccount('Alice');

        // To create an address we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const accountAddress = (await account.generateEd25519Addresses(1))[0];
        const address = Utils.parseBech32Address(accountAddress.address);

        console.log(`Generated address:`, address.toString());
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
