// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/send_micro_transaction.ts

// This example sends a micro transaction
async function run() {
    initLogger();
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

        const wallet = await Wallet.create({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await wallet.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD as string,
        );

        // Replace with the address of your choice!
        const address =
            'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';
        const amount = BigInt(1);

        const transaction = await wallet.send(amount, address, {
            allowMicroAmount: true,
        });

        console.log(`Transaction sent: ${transaction.transactionId}`);

        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
