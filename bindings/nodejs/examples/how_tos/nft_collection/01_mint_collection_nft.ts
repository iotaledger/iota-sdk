// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MintNftParams, NftId, Utils, Wallet, Irc27Metadata } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// The NFT collection size
const NFT_COLLECTION_SIZE = 150;
// Mint NFTs in chunks since the transaction size is limited
const NUM_NFTS_MINTED_PER_TRANSACTION = 50;

// In this example we will mint the issuer NFT for the NFT collection.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/01_mint_collection_nft.ts <issuer_nft_id>
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

        // Get the account we generated with `how_tos/accounts_and_addresses/create-account`
        const account = await wallet.getAccount('Alice');

        await account.sync();
        console.log(`Account synced!`);

        // Get the id we generated with `00_mint_issuer_nft`
        const issuerNftId: NftId = process.argv[2];

        const bech32Hrp = await (await wallet.getClient()).getBech32Hrp();
        const issuer = Utils.nftIdToBech32(issuerNftId, bech32Hrp);

        const nftMintParams = [];
        // Create the metadata with another index for each
        for (let index = 0; index < NFT_COLLECTION_SIZE; index++) {
            const params: MintNftParams = {
                immutableMetadata: getImmutableMetadata(index).asHex(),
                // The NFT address from the NFT we minted in mint_issuer_nft example
                issuer,
            };
            nftMintParams.push(params);
        }

        for (
            let i = 0;
            i < nftMintParams.length;
            i += NUM_NFTS_MINTED_PER_TRANSACTION
        ) {
            const chunk = nftMintParams.slice(
                i,
                i + NUM_NFTS_MINTED_PER_TRANSACTION,
            );

            console.log(
                `Minting ${chunk.length} NFTs... (${
                    i + chunk.length
                }/${NFT_COLLECTION_SIZE})`,
            );
            const transaction = await account.mintNfts(chunk);

            // Wait for transaction to get included
            const blockId = await account.retryTransactionUntilIncluded(
                transaction.transactionId,
            );
            console.log(
                `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            // Sync so the new outputs are available again for new transactions
            await account.sync();
        }

        // After the NFTs are minted, the issuer nft can be sent to the so called "null address"
        // 0x0000000000000000000000000000000000000000000000000000000000000000 (for smr:
        // smr1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqy8f002) or burned, to
        // prevent minting any further NFTs in the future. Sending it to the null address makes it still available to get
        // its metadata.
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

function getImmutableMetadata(index: number) {
    return new Irc27Metadata(
        'video/mp4',
        'https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT',
        `Shimmer OG NFT ${index}`,
    )
        .withDescription(
            'The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation \
        to celebrate the official launch of the Shimmer Network.',
        )
        .withIssuerName('IOTA Foundation')
        .withCollectionName('Shimmer OG');
}

run();
