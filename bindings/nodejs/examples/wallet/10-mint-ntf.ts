// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    Ed25519Address,
    IssuerFeature,
    MintNftParams,
    SenderFeature,
    Utils,
} from '@iota/sdk';

import { getUnlockedManager } from './account-manager';

// The owner address of the first NFT we'll mint
const NFT1_OWNER_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';
// The metadata of the first minted NFT
const NFT1_METADATA = stringToHex('some NFT metadata');
// The immutable metadata of the first minted NFT
const NFT1_IMMUTABLE_METADATA = stringToHex('some NFT immutable metadata');
// The tag of the first minted NFT
const NFT1_TAG = stringToHex('some NFT tag');
// The base coin amount we sent with the second NFT
const NFT2_AMOUNT = '1000000';

// In this example we will mint an NFT in two different ways.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/10-mint-nft.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();
        const nftsBefore = balance.nfts;

        // We send from the first address in the account.
        const senderAddress = (await account.addresses())[0].address;

        console.log('Sending the minting transaction for NFT 1...');

        const params: MintNftParams = {
            address: NFT1_OWNER_ADDRESS, // Remove or change to senderAddress to send to self
            sender: senderAddress,
            metadata: NFT1_METADATA,
            tag: NFT1_TAG,
            issuer: senderAddress,
            immutableMetadata: NFT1_IMMUTABLE_METADATA,
        };
        const prepared = await account.prepareMintNfts([params]);

        let transaction = await prepared.send();
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        let blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
        console.log('Minted NFT 1');

        // Build an NFT manually by using the `NftOutputBuilder`
        const client = await manager.getClient();

        const hexAddress = Utils.bech32ToHex(senderAddress);
        const output = await client.buildNftOutput({
            amount: NFT2_AMOUNT,
            nftId: '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new AddressUnlockCondition(new Ed25519Address(hexAddress)),
            ],
            immutableFeatures: [
                new IssuerFeature(new Ed25519Address(hexAddress)),
            ],
            features: [new SenderFeature(new Ed25519Address(hexAddress))],
        });

        console.log('Sending minting transaction for NFT 2...');

        transaction = await account.sendOutputs([output]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        console.log('Minted NFT 2');

        // Ensure the account is synced after minting.
        balance = await account.sync();
        const nftsAfter = balance.nfts;

        console.log('New owned NFTs:', nftsBefore.length, nftsAfter.length);
        for (const nftId of nftsAfter) {
            if (!nftsBefore.includes(nftId)) {
                console.log(`- ${nftId}`);
            }
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

function stringToHex(str: string): string {
    let hexEncoded = '0x';
    for (let i = 0; i < str.length; i++) {
        const hex = str.charCodeAt(i).toString(16);
        hexEncoded += hex.padStart(2, '0');
    }
    return hexEncoded;
}

run();
