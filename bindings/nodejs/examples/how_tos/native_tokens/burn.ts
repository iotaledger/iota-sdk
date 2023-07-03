// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from '../../wallet/common';

// The minimum available native token amount to search for in the account, 11 hex encoded.
const MIN_AVAILABLE_AMOUNT = '0xB';
// The amount of the native token to burn, 1 hex encoded.
const BURN_AMOUNT = '0x1';

// In this example we will burn a native token. This will not increase the melted supply in the foundry,
// therefore the foundry output is also not required. But this will also make it impossible to destroy the foundry
// output that minted it.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/burn.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        // Get a token with sufficient balance
        const tokenId = balance.nativeTokens.find(
            (t) => Number(t.available) >= Number(MIN_AVAILABLE_AMOUNT),
        )?.tokenId;

        let token = balance.nativeTokens.find(
            (nativeToken) =>
                nativeToken.tokenId == tokenId &&
                Number(nativeToken.available) >= Number(MIN_AVAILABLE_AMOUNT),
        );
        if (!token) {
            throw new Error(
                `Native token '${tokenId}' doesn't exist or there's not at least '${Number(
                    MIN_AVAILABLE_AMOUNT,
                )}' tokens of it in account 'Alice'`,
            );
        }

        console.log(`Balance before burning: ${parseInt(token.available)}`);

        // Burn a native token
        const transaction = await account
            .prepareBurnNativeToken(token.tokenId, BURN_AMOUNT)
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

        token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == tokenId,
        );
        if (token) {
            console.log(`Balance after burning: ${parseInt(token.available)}`);
        } else {
            console.log(`No remaining tokens`);
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
