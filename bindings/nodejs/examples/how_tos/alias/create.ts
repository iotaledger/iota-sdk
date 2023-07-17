// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias/create.ts

// In this example we create alias.
async function run() {
    initLogger();
    if (!process.env.FAUCET_URL) {
        throw new Error('.env FAUCET_URL is undefined, see .env.example');
    }
    if (!process.env.WALLET_DB_PATH) {
        throw new Error('.env WALLET_DB_PATH is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
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
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        console.log('Sending the create-alias transaction...');

        // Create an alias
        const transaction = await account
            .prepareCreateAliasOutput()
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        balance = await account.sync();
        console.log(`Aliases AFTER:\n`, balance.aliases);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
