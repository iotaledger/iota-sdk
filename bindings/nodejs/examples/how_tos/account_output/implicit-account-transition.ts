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
// yarn run-example ./how_tos/account_output/implicit-account-transition.ts

// In this example we transition an implicit account to an account.
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

        // Need to sync the wallet with implicit accounts option enabled.
        let balance = await wallet.sync({ syncImplicitAccounts: true });

        let implicitAccounts = await wallet.implicitAccounts();
        if (implicitAccounts.length == 0) {
            throw new Error(`No implicit account available`);
        }

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        console.log('Sending the transition transaction...');

        // Transition to the account output.
        const transaction = await wallet.implicitAccountTransition(
            implicitAccounts[0].outputId,
        );

        console.log(`Transaction sent: ${transaction.transactionId}`);

        await wallet.waitForTransactionAcceptance(transaction.transactionId);
        console.log(
            `Tx accepted: ${process.env.EXPLORER_URL}/transactions/${transaction.transactionId}`,
        );

        balance = await wallet.sync();
        console.log(`Accounts:\n`, balance.accounts);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
