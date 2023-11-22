// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias/destroy.ts

// In this example we destroy alias.
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

        if (balance.aliases.length == 0) {
            throw new Error(`No Alias available in account 'Alice'`);
        }

        // We try to destroy the first alias in the account
        const aliasId = balance.aliases[0];

        console.log(
            `Aliases BEFORE destroying (${balance.aliases.length}):\n`,
            balance.aliases,
        );

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        console.log('Sending the destroy-alias transaction...');

        // Destroy an alias
        const transaction = await account
            .prepareDestroyAlias(aliasId)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(`Destroyed alias ${aliasId}`);

        balance = await account.sync();
        console.log(
            `Aliases AFTER destroying (${balance.aliases.length}):\n`,
            balance.aliases,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
