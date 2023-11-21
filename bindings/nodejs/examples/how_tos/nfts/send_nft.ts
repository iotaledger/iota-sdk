// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SendNftParams, Wallet } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// The address to send the NFT to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send an NFT (Non-fungible token).
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-account` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/nfts/send_nft.ts
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
        const balance = await account.sync();

        if (balance.nfts.length == 0) {
            throw new Error('No available NFTs');
        }

        const nftId = balance.nfts[0];

        const outputs: SendNftParams[] = [
            {
                address: RECV_ADDRESS,
                nftId,
            },
        ];

        // Send the full NFT output to the specified address
        const transaction = await account.sendNft(outputs);

        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );

        // To send an NFT with expiration unlock condition prepareOutput() can be used like this:
        // const output = await account.prepareOutput({
        //     recipientAddress: 'rms1qz6aj69rumk3qu0ra5ag6p6kk8ga3j8rfjlaym3wefugs3mmxgzfwa6kw3l',
        //     amount: "47000",
        //     unlocks: {
        //         expirationUnixTime: 1677065933
        //     },
        //     assets: {
        //         nftId: '0x447b20b81e2311a6c16a32eaeda2f2f2472c4b43ed4ffc80a0c0f850130fc4bb',
        //     },
        //     storageDeposit: { returnStrategy: 'Gift' }
        // });

        // const transaction = await account.sendOutputs([output]);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
