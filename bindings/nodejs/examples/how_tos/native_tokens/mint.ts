// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MintNativeTokenParams, utf8ToHex } from '@iota/sdk';

import { getUnlockedWallet } from '../../wallet/common';

// The circulating supply of the native token. `100` hex encoded
const CIRCULATING_SUPPLY = '0x64';
// The maximum supply of the native token. `100` hex encoded
const MAXIMUM_SUPPLY = '0x64';

// In this example we will mint a native token.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/09-mint-native-token.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        const balance = await account.sync();

        // We can first check if we already have an alias in our account, because an alias can have many foundry outputs and therefore we can reuse an existing one
        if (balance.aliases.length > 0) {
            // If we don't have an alias, we need to create one
            const transaction = await account
                .prepareCreateAliasOutput()
                .then((prepared) => prepared.send());
            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            const blockId = await account.retryTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            await account.sync();
            console.log('Account synced');
        }

        console.log('Preparing minting transaction...');

        // If we omit the AccountAddress field the first address of the account is used by default
        const params: MintNativeTokenParams = {
            circulatingSupply: CIRCULATING_SUPPLY,
            maximumSupply: MAXIMUM_SUPPLY,
            foundryMetadata: utf8ToHex('Hello, World!'),
        };

        const prepared = await account.prepareMintNativeToken(params);
        const transaction = await prepared.send();

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        console.log(`Minted token: ${prepared.tokenId()}`);

        // Ensure the account is synced after minting.
        await account.sync();
        console.log('Account synced');
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
