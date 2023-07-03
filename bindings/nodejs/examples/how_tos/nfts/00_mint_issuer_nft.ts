// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@iota/sdk';

// In this example we will mint the issuer NFT for the NFT collection.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/00_mint_issuer_nft.ts
async function run() {
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
