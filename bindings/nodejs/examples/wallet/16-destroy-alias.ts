// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from './common';

// In this example we will try to destroy the first alias there is in the account. This is only possible if possible
// foundry outputs have circulating supply of 0.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/16-destroy-alias.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        if (balance.aliases.length == 0) {
            throw new Error(
                `No Alias available in account '${process.env.ACCOUNT_ALIAS_1}'`,
            );
        }

        // We try to destroy the first alias in the account
        const aliasId = balance.aliases[0];

        console.log(`Aliases BEFORE destroying:\n`, balance.aliases);
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
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(`Destroyed alias ${aliasId}`);

        balance = await account.sync();
        console.log(`Aliases AFTER destroying:\n`, balance.aliases);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
