// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedManager } from '../../wallet/account-manager';

// The amount of native tokens to mint, 10 hex encoded.
const MINT_AMOUNT = '0xA';

// In this example we will mint an existing native token with its foundry.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/12-increase-native-token-supply.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        // Find first foundry and corresponding token id
        const tokenId = balance.foundries[0];

        let token = balance.nativeTokens.find(
            (nativeToken) => nativeToken.tokenId == tokenId,
        );
        if (token == null) {
            throw new Error(
                `Couldn't find native token '${tokenId}' in the account`,
            );
        }

        console.log(`Balance before minting:`, token.available);

        // Mint some more native tokens
        const transaction = await account
            .prepareIncreaseNativeTokenSupply(token.tokenId, MINT_AMOUNT)
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
        console.log(`Balance after minting:`, token?.available);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
