// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedManager } from './account-manager';

// The native token id. Replace it with a TokenId that is available in the account, the foundry output which minted it,
// also needs to be available. You can check this by running the `get-balance` example. You can mint a new native token
// by running the `mint-native-token` example.
// eslint-disable-next-line prefer-const
let TOKEN_ID =
    '0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000';
// The amount of native tokens to melt, 10 hex encoded.
const MELT_AMOUNT = '0xA';

// In this example we will melt an existing native token with its foundry.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/11-decrease-native-token-supply.ts
async function run() {
    try {
        if (
            TOKEN_ID ==
            '0x08847bd287c912fadedb6bf38900bda9f2d377b75b2a0bece8738699f56ebca4130100000000'
        ) {
            console.log(
                'You need to change the TOKEN_ID constant before you can run this example successfully!',
            );
            return;
        }

        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        let token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == TOKEN_ID,
        );
        if (token == null) {
            throw new Error(
                `Couldn't find native token '${TOKEN_ID}' in the account`,
            );
        }

        console.log(`Balance BEFORE melting:\n`, token);

        console.log('Sending the melting transaction...');

        // Melt some of the circulating supply
        const transaction = await account
            .prepareDecreaseNativeTokenSupply(token.tokenId, MELT_AMOUNT)
            .then((prepared) => prepared.finish());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(
            `Melted ${Number(MELT_AMOUNT)} native tokens (${token.tokenId})`,
        );

        balance = await account.sync();
        token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == TOKEN_ID,
        );
        console.log(`Balance AFTER melting:\n`, token);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
