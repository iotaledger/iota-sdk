// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MintNativeTokenParams } from '@iota/sdk';

import { getUnlockedWallet } from './common';

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
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        console.log('Sending alias output transaction...');

        // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs
        let transaction = await account
            .prepareCreateAliasOutput()
            .then((prepared) => prepared.send());
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        let blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        await account.sync();
        console.log('Account synced');

        console.log('Sending the minting transaction...');

        // If we omit the AccountAddress field the first address of the account is used by default
        const params: MintNativeTokenParams = {
            circulatingSupply: CIRCULATING_SUPPLY,
            maximumSupply: MAXIMUM_SUPPLY,
        };

        const prepared = await account.prepareMintNativeToken(params);
        transaction = await prepared.send();

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
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
