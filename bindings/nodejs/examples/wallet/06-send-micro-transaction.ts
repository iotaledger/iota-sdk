// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SendAmountParams } from '@iota/sdk';

import { getUnlockedWallet } from './common';

// The base coin micro amount to send
const SEND_MICRO_AMOUNT = '1';
// The address to send the coins to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send an amount below the minimum storage deposit.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/06-send-micro-transaction.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        await account.sync();

        console.log(
            `Sending '${SEND_MICRO_AMOUNT}' coin(s) to '${RECV_ADDRESS}'...`,
        );
        const outputs: SendAmountParams[] = [
            { address: RECV_ADDRESS, amount: SEND_MICRO_AMOUNT },
        ];

        const transaction = await account.sendAmount(outputs, {
            allowMicroAmount: true,
        });
        console.log(`Transaction sent: ${transaction.transactionId}`);

        // Wait for transaction to get included
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(
            `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
