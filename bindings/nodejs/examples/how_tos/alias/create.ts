// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias/create.ts

// In this example we create alias.
async function run() {
    initLogger();
    for (const envVar of [
        'WALLET_DB_PATH',
        'STRONGHOLD_PASSWORD',
        'EXPLORER_URL',
    ]) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    try {
        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        console.log(`Aliases BEFORE:\n`, balance.aliases);

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        console.log('Sending the create-alias transaction...');

        // Create an alias
        const transaction = await account.createAliasOutput();

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        balance = await account.sync();
        console.log(`Aliases AFTER:\n`, balance.aliases);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
