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
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // May want to ensure the wallet is synced before sending a transaction.
        let balance = await wallet.sync();

        if (balance.accounts.length == 0) {
            throw new Error(`No Account output available in account 'Alice'`);
        }

        // We try to destroy the first account output in the wallet
        const accountId = balance.accounts[0];

        console.log(
            `Accounts BEFORE destroying (${balance.accounts.length}):\n`,
            balance.accounts,
        );

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        console.log('Sending the destroy-account transaction...');

        // Destroy an account output
        const transaction = await wallet
            .prepareDestroyAccount(accountId)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(`Destroyed account output ${accountId}`);

        balance = await wallet.sync();
        console.log(
            `Accounts AFTER destroying (${balance.accounts.length}):\n`,
            balance.accounts,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
