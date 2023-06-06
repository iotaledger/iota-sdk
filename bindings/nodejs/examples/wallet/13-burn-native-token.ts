// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from './common';

// The native token id. Replace it with a TokenId that is available in the account, the foundry output which minted it,
// also needs to be available. You can check this by running the `get_balance` example. You can mint a new native token
// by running the `mint_native_token` example.
// eslint-disable-next-line prefer-const
let TOKEN_ID =
    '0x08dc44610c24f32f26330440f3f0d4afb562a8dfd81afe7c2f79024f8f1b9e21940100000000';
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
// yarn run-example ./wallet/13-burn-native-token.ts
async function run() {
    try {
        if (
            TOKEN_ID ==
            '0x086f7011adb53642e8ed7db230c2307fe980f4aff2685c22f7c84a61ec558f691b0200000000'
        ) {
            throw new Error(
                'You need to change the TOKEN_ID constant before you can run this example successfully!',
            );
        }

        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        let token = balance.nativeTokens.find(
            (nativeToken) =>
                nativeToken.tokenId == TOKEN_ID &&
                Number(nativeToken.available) >= Number(MIN_AVAILABLE_AMOUNT),
        );
        if (!token) {
            throw new Error(
                `"Native token '${TOKEN_ID}' doesn't exist or there's not at least '${Number(
                    MIN_AVAILABLE_AMOUNT,
                )}' tokens of it in account '${process.env.ACCOUNT_ALIAS_1}'"`,
            );
        }

        console.log(`Balance BEFORE burning:\n`, token);
        console.log(`Sending the burning transaction...`);

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
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(
            `Burned ${Number(BURN_AMOUNT)} native token(s) (${token.tokenId})`,
        );

        balance = await account.sync();

        console.log(`Balance AFTER burning:`);
        token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == TOKEN_ID,
        );
        if (token) {
            console.log(token);
        } else {
            console.log(`No remaining tokens`);
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
