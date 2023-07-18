// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger, Utils } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/alias_wallet/transaction.ts

// In this example we send funds from an alias wallet.
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const syncOptions = {
            alias: {
                basicOutputs: true,
            },
        };

        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        const balance = await account.sync(syncOptions);

        const totalBaseTokenBalance = balance.baseCoin.total;
        console.log(
            `Balance before sending funds from alias: ${totalBaseTokenBalance}`,
        );

        const aliasId = balance.aliases[0];
        console.log(`Alias Id: ${aliasId}`);

        // Get Alias address
        const aliasAddress = Utils.aliasIdToBech32(
            aliasId,
            await (await wallet.getClient()).getBech32Hrp(),
        );

        // Find first output unlockable by the alias address
        const queryParameters = [
            {
                address: aliasAddress,
            },
        ];
        const input = (
            await (await wallet.getClient()).basicOutputIds(queryParameters)
        ).items[0];

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
        const transaction = await account.sendWithParams(params, options);
        await account.retryTransactionUntilIncluded(transaction.transactionId);
        console.log(
            `Transaction with custom input: https://explorer.iota.org/testnet/transaction/${transaction.transactionId}`,
        );

        const totalBaseTokenBalanceAfter = (await account.sync(syncOptions))
            .baseCoin.total;
        console.log(
            `Balance after sending funds from alias: ${totalBaseTokenBalanceAfter}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
