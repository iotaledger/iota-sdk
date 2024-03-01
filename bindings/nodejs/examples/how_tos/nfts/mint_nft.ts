// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    Ed25519Address,
    IssuerFeature,
    MintNftParams,
    SenderFeature,
    utf8ToHex,
    Utils,
    Wallet,
    Irc27Metadata,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// The owner address of the first NFT we'll mint
const NFT1_OWNER_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';
// The metadata of the first minted NFT
const NFT1_METADATA = utf8ToHex('some NFT metadata');
// The tag of the first minted NFT
const NFT1_TAG = utf8ToHex('some NFT tag');
// The base coin amount we sent with the second NFT
const NFT2_AMOUNT = '1000000';

// In this example we will mint a new nft.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/wallet/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/mint_nft.ts
async function run() {
    try {
        for (const envVar of [
            'STRONGHOLD_PASSWORD',
            'WALLET_DB_PATH',
            'EXPLORER_URL',
        ])
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        // We send from the address in the wallet.
        const senderAddress = await wallet.address();

        // We need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        const metadata = new Irc27Metadata(
            'video/mp4',
            'https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT',
            'Shimmer OG NFT',
        ).withDescription('The original Shimmer NFT');

        const params: MintNftParams = {
            address: NFT1_OWNER_ADDRESS, // Remove or change to senderAddress to send to self
            sender: senderAddress,
            metadata: NFT1_METADATA,
            tag: NFT1_TAG,
            issuer: senderAddress,
            immutableMetadata: metadata.asHex(),
        };
        let transaction = await wallet.mintNfts([params]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get accepted
        await wallet.waitForTransactionAcceptance(transaction.transactionId);

        console.log(
            `Tx accepted: ${process.env.EXPLORER_URL}/transaction/${transaction.transactionId}`,
        );
        console.log('Minted NFT 1');

        // Build an NFT manually by using the `NftOutputBuilder`
        const client = await wallet.getClient();

        const hexAddress = Utils.bech32ToHex(senderAddress);
        const output = await client.buildNftOutput({
            amount: NFT2_AMOUNT,
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(Utils.bech32ToHex(NFT1_OWNER_ADDRESS)),
                ),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
            ],
            features: [new SenderFeature(new Ed25519Address(hexAddress))],
        });

        transaction = await wallet.sendOutputs([output]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get accepted
        await wallet.waitForTransactionAcceptance(transaction.transactionId);

        console.log(
            `Tx accepted: ${process.env.EXPLORER_URL}/transaction/${transaction.transactionId}`,
        );

        console.log('Minted NFT 2');

        // Ensure the wallet is synced after minting.
        await wallet.sync();
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
