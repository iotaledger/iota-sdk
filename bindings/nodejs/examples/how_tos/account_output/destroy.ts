// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_output/destroy.ts

// In this example we destroy an account output.
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

        if (balance.accounts.length == 0) {
            throw new Error(`No Account output available in account 'Alice'`);
        }

        // We try to destroy the first account output in the account
        const accountId = balance.accounts[0];

        console.log(
            `Accounts BEFORE destroying (${balance.accounts.length}):\n`,
            balance.accounts,
        );

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        console.log('Sending the destroy-account transaction...');

        // Destroy an account output
        const transaction = await account
            .prepareDestroyAccount(accountId)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(`Destroyed account output ${accountId}`);

        balance = await account.sync();
        console.log(
            `Accounts AFTER destroying (${balance.accounts.length}):\n`,
            balance.accounts,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
