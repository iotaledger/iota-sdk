// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from '../../wallet/common';

// The amount of native tokens to melt.
const MELT_AMOUNT = BigInt(10);

// In this example we will melt an existing native token with its foundry.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/wallet/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/melt.ts
async function run() {
    for (const envVar of ['EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // May want to ensure the wallet is synced before sending a transaction.
        let balance = await wallet.sync();

        if (balance.foundries.length == 0) {
            throw new Error(`No Foundry available in account 'Alice'`);
        }

        // Find first foundry and corresponding token id
        const tokenId = balance.foundries[0];

        let token = balance.nativeTokens[tokenId];
        if (token == null) {
            throw new Error(
                `Couldn't find native token '${tokenId}' in the wallet`,
            );
        }

        console.log(`Balance before melting: ${token.available}`);

        // Melt some of the circulating supply
        const transaction = await wallet.meltNativeToken(tokenId, MELT_AMOUNT);

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get accepted
        await wallet.waitForTransactionAcceptance(transaction.transactionId);

        console.log(
            `Tx accepted: ${process.env.EXPLORER_URL}/transactions/${transaction.transactionId}`,
        );

        balance = await wallet.sync();
        token = balance.nativeTokens[tokenId];
        if (token == null) {
            throw new Error(
                `Couldn't find native token '${tokenId}' in the wallet`,
            );
        }
        console.log(`Balance after melting: ${token.available}`);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
