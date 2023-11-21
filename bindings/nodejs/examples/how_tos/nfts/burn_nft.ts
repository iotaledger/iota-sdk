// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// In this example we will burn an existing nft output.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/burn_nft.ts
async function run() {
    try {
        for (const envVar of [
            'WALLET_DB_PATH',
            'STRONGHOLD_PASSWORD',
            'EXPLORER_URL',
        ]) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }

        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        // We need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        if (balance.nfts.length == 0) {
            throw new Error(
                `No NFT available in account '${process.env.ACCOUNT_ALIAS_1}'`,
            );
        }
        // Get the first nft
        const nftId = balance.nfts[0];

        console.log(`Balance BEFORE burning:\n`, balance);

        // Burn an NFT
        const transaction = await account
            .prepareBurnNft(nftId)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log(`Burned NFT ${nftId}`);

        balance = await account.sync();
        console.log(`Balance AFTER burning:\n`, balance);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
