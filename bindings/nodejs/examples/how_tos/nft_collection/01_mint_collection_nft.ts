// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MintNftParams, NftAddress, NftId, Utils, Wallet } from '@iota/sdk';

// The NFT collection size
const NFT_COLLECTION_SIZE = 10000;
// Mint NFTs in chunks since the transaction size is limited
const NUM_NFTS_MINTED_PER_TRANSACTION = 50;

// In this example we will mint the issuer NFT for the NFT collection.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/01_mint_collection_nft.ts
async function run() {
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }
        let issuerNftId: NftId = "";
        // "missing example argument: ISSUER_NFT_ID"

        // Create the wallet
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        const bech32Hrp = await (await wallet.getClient()).getBech32Hrp();
        
        const nftMintParams = [];
        // Create the metadata with another index for each
        for (let index = 0; index < NFT_COLLECTION_SIZE; index++) {
            const params: MintNftParams = {
                immutableMetadata: getImmutableMetadata(index, issuerNftId),
                // The NFT address from the NFT we minted in mint_issuer_nft example
                issuer: Utils.nftIdToBech32(issuerNftId, bech32Hrp),
            };
            nftMintParams.push(params);
        }

        for (let i = 0; i < nftMintParams.length; i += NUM_NFTS_MINTED_PER_TRANSACTION) {
            const chunk = nftMintParams.slice(i, i + NUM_NFTS_MINTED_PER_TRANSACTION);

            console.log(`Minting ${chunk.length} NFTs...`);
            const prepared = await account.prepareMintNfts(chunk);
            const transaction = await prepared.send();
            
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

function getImmutableMetadata(index: number, issuerNftId: NftId) {
    return `{
        "standard":"IRC27",
        "version":"v1.0",
        "type":"video/mp4",
        "uri":"ipfs://wrongcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5Ywrong",
        "name":"Shimmer OG NFT ${index}",
        "description":"The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.",
        "issuerName":"IOTA Foundation",
        "collectionId":"${issuerNftId}",
        "collectionName":"Shimmer OG"
    }`;
}

run();
