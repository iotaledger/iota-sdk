// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Utils, Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias-wallet/request-funds.ts

// In this example we request funds to an alias wallet.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH', 'FAUCET_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const faucetUrl = process.env.FAUCET_URL as string;

        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `create_wallet`
        const account = await wallet.getAccount('Alice');

        const balance = await account.sync();

        const totalBaseTokenBalance = balance.baseCoin.total;
        console.log(
            `Balance before requesting funds on alias address: ${totalBaseTokenBalance}`,
        );

        const aliasId = balance.aliases[0];
        console.log(`Alias Id: ${aliasId}`);

        // Get Alias address
        const aliasAddress = Utils.aliasIdToBech32(
            aliasId,
            await (await wallet.getClient()).getBech32Hrp(),
        );
        const faucetResponse = await (
            await wallet.getClient()
        ).requestFundsFromFaucet(faucetUrl, aliasAddress);
        console.log(faucetResponse);

        await new Promise((resolve) => setTimeout(resolve, 10000));

        const syncOptions = {
            alias: {
                basicOutputs: true,
            },
        };
        const totalBaseTokenBalanceAfter = (await account.sync(syncOptions))
            .baseCoin.total;
        console.log(
            `Balance after requesting funds on alias address: ${totalBaseTokenBalanceAfter}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
