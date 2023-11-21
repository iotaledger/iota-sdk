// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from '../../wallet/common';

// In this example we will try to destroy the first foundry there is in the account. This is only possible if its
// circulating supply is 0 and no native tokens were burned.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/destroy-foundry.ts
async function run() {
    for (const envVar of ['EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        if (balance.foundries.length == 0) {
            throw new Error(`No Foundry available in account 'Alice'`);
        }

        // We try to destroy the first foundry in the account
        const foundry = balance.foundries[0];

        console.log(`Foundries before destroying: ${balance.foundries.length}`);

        // Burn a foundry
        const transaction = await account
            .prepareDestroyFoundry(foundry)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        balance = await account.sync();
        console.log(`Foundries after destroying: ${balance.foundries.length}`);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
