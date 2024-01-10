// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger, Utils } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/account_wallet/transaction.ts

// In this example we send funds from an account wallet.
async function run() {
    initLogger();
    try {
        for (const envVar of ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }

        const syncOptions = {
            account: {
                basicOutputs: true,
            },
        };

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const balance = await wallet.sync(syncOptions);

        const totalBaseTokenBalance = balance.baseCoin.total;
        console.log(
            `Balance before sending funds from account: ${totalBaseTokenBalance}`,
        );

        const accountId = balance.accounts[0];
        console.log(`Account Id: ${accountId}`);

        const client = await wallet.getClient();

        // Get Account address
        const accountAddress = Utils.accountIdToBech32(
            accountId,
            await client.getBech32Hrp(),
        );

        // Find first output unlockable by the account address
        const queryParameters = {
            address: accountAddress,
        };
        const input = (await client.basicOutputIds(queryParameters)).items[0];

        const params = [
            {
                address:
                    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu',
                amount: BigInt(1000000),
            },
        ];
        const options = {
            mandatoryInputs: [input],
            allowMicroAmount: false,
        };
        const transaction = await wallet.sendWithParams(params, options);
        await wallet.reissueTransactionUntilIncluded(transaction.transactionId);
        console.log(
            `Transaction with custom input: https://explorer.iota.org/testnet/transaction/${transaction.transactionId}`,
        );

        const totalBaseTokenBalanceAfter = (await wallet.sync(syncOptions))
            .baseCoin.total;
        console.log(
            `Balance after sending funds from account: ${totalBaseTokenBalanceAfter}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
