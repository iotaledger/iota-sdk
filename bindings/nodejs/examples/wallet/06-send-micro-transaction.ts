// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SendParams } from '@iota/sdk';

import { getUnlockedWallet } from './common';

// The base coin micro amount to send
const SEND_MICRO_AMOUNT = BigInt(1);
// The address to send the coins to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send an amount below the minimum output amount.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/06-send-micro-transaction.ts
async function run() {
    try {
        for (const envVar of ['EXPLORER_URL']) {
            if (!(envVar in process.env)) {
                throw new Error(
                    `.env ${envVar} is undefined, see .env.example`,
                );
            }
        }
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // May want to ensure the wallet is synced before sending a transaction.
        await wallet.sync();

        console.log(
            `Sending '${SEND_MICRO_AMOUNT}' coin(s) to '${RECV_ADDRESS}'...`,
        );
        const params: SendParams[] = [
            { address: RECV_ADDRESS, amount: SEND_MICRO_AMOUNT },
        ];

        const transaction = await wallet.sendWithParams(params, {
            allowMicroAmount: true,
        });
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
