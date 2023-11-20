// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils, Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_wallet/request-funds.ts

// In this example we request funds to an account wallet.
async function run() {
    initLogger();
    if (!process.env.FAUCET_URL) {
        throw new Error('.env FAUCET_URL is undefined, see .env.example');
    }
    try {
        const faucetUrl = process.env.FAUCET_URL;

        // Create the wallet
        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const balance = await wallet.sync();

        const totalBaseTokenBalance = balance.baseCoin.total;
        console.log(
            `Balance before requesting funds on account address: ${totalBaseTokenBalance}`,
        );

        const accountId = balance.accounts[0];
        console.log(`Account Id: ${accountId}`);

        // Get Account address
        const accountAddress = Utils.accountIdToBech32(
            accountId,
            await (await wallet.getClient()).getBech32Hrp(),
        );
        const faucetResponse = await (
            await wallet.getClient()
        ).requestFundsFromFaucet(faucetUrl, accountAddress);
        console.log(faucetResponse);

        await new Promise((resolve) => setTimeout(resolve, 10000));

        const syncOptions = {
            account: {
                basicOutputs: true,
            },
        };
        const totalBaseTokenBalanceAfter = (await wallet.sync(syncOptions))
            .baseCoin.total;
        console.log(
            `Balance after requesting funds on account address: ${totalBaseTokenBalanceAfter}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
