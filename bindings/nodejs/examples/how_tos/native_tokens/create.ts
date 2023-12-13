// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { CreateNativeTokenParams, Irc30Metadata } from '@iota/sdk';

import { getUnlockedWallet } from '../../wallet/common';

// The circulating supply of the native token.
const CIRCULATING_SUPPLY = BigInt(100);
// The maximum supply of the native token.
const MAXIMUM_SUPPLY = BigInt(100);

// In this example we will create a native token.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/create.ts
async function run() {
    for (const envVar of ['EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        const balance = await wallet.sync();

        // We can first check if we already have an account output in our account, because an account output can have
        // many foundry outputs and therefore we can reuse an existing one
        if (balance.accounts.length == 0) {
            // If we don't have an account output, we need to create one
            const transaction = await wallet
                .prepareCreateAccountOutput()
                .then((prepared) => prepared.send());
            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            const blockId = await wallet.reissueTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            await wallet.sync();
            console.log('Wallet synced');
        }

        console.log('Preparing transaction to create native token...');

        const metadata = new Irc30Metadata(
            'My Native Token',
            'MNT',
            10,
        ).withDescription('A native token to test the iota-sdk.');

        // If we omit the AccountAddress field the wallet address is used by default
        const params: CreateNativeTokenParams = {
            circulatingSupply: CIRCULATING_SUPPLY,
            maximumSupply: MAXIMUM_SUPPLY,
            foundryMetadata: metadata.asHex(),
        };

        const prepared = await wallet.prepareCreateNativeToken(params);
        const transaction = await prepared.send();

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        console.log(`Created token: ${prepared.tokenId()}`);

        // Ensure the account is synced after creating the native token.
        await wallet.sync();
        console.log('Wallet synced');
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
