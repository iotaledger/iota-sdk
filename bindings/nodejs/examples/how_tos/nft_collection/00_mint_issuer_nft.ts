// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    MintNftParams,
    NftId,
    NftOutput,
    RegularTransactionEssence,
    TransactionPayload,
    utf8ToHex,
    Utils,
    Wallet,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// In this example we will mint the issuer NFT for the NFT collection.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/00_mint_issuer_nft.ts
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

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount('Alice');

        await account.sync();
        console.log(`Account synced!`);

        // Issue the minting transaction and wait for its inclusion
        console.log(`Sending NFT minting transaction...`);
        const params: MintNftParams = {
            immutableMetadata: utf8ToHex(
                'This NFT will be the issuer from the awesome NFT collection',
            ),
        };
        const transaction = await account.mintNfts([params]);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        const essence: RegularTransactionEssence = (
            transaction.payload as TransactionPayload
        ).essence as RegularTransactionEssence;
        essence.outputs.forEach((output, outputIndex) => {
            if (output instanceof NftOutput) {
                const nftOutput = output as NftOutput;

                // New minted NFT id is empty in the output
                if (
                    nftOutput.getNftId() ===
                    '0x0000000000000000000000000000000000000000000000000000000000000000'
                ) {
                    const outputId = Utils.computeOutputId(
                        transaction.transactionId,
                        outputIndex,
                    );
                    const nftId: NftId = Utils.computeNftId(outputId);
                    console.log(`New minted NFT id: ${nftId}`);
                }
            }
        });
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
