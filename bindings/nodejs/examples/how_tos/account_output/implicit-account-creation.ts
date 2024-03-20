// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/wallet/create-wallet` example!
//
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_output/implicit-account-creation.ts

// In this example we create an implicit account creation address.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    try {
        // Create the wallet
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the implicit account address.
        const implicitAccountCreationAddress =
            await wallet.implicitAccountCreationAddress();
        console.log(
            `Implicit account creation address: ${implicitAccountCreationAddress}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
