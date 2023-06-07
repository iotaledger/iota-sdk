// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getUnlockedWallet } from './common';

// In this example we will burn an existing nft output.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/14-burn-nft.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
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

        // Burn a native token
        const transaction = await account
            .prepareBurnNft(nftId)
            .then((prepared) => prepared.send());

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
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
