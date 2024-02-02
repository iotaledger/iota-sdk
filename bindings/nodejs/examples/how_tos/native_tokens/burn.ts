// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from '../../wallet/common';

// The minimum available native token amount to search for in the wallet.
const MIN_AVAILABLE_AMOUNT = BigInt(11);
// The amount of the native token to burn.
const BURN_AMOUNT = BigInt(1);

// In this example we will burn a native token. This will not increase the melted supply in the foundry,
// therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
// output that minted it.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/wallet/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/burn.ts
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

        // Get a token with sufficient balance
        const tokenId = balance.nativeTokens.find(
            (t) => t.available >= MIN_AVAILABLE_AMOUNT,
        )?.tokenId;

        let token = balance.nativeTokens.find(
            (nativeToken) =>
                nativeToken.tokenId == tokenId &&
                Number(nativeToken.available) >= MIN_AVAILABLE_AMOUNT,
        );
        if (!token) {
            throw new Error(
                `Native token '${tokenId}' doesn't exist or there's not at least '${Number(
                    MIN_AVAILABLE_AMOUNT,
                )}' tokens of it in the wallet`,
            );
        }

        console.log(`Balance before burning: ${token.available}`);

        // Burn a native token
        const transaction = await wallet
            .prepareBurnNativeToken(token.tokenId, BURN_AMOUNT)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        balance = await wallet.sync();

        token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == tokenId,
        );
        if (token) {
            console.log(`Balance after burning: ${token.available}`);
        } else {
            console.log(`No remaining tokens`);
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
