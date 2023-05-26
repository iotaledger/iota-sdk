// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AddressUnlockCondition, Client, Ed25519Address, OutputsToClaim, TimelockUnlockCondition, Utils, Wallet, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/claim_transaction.ts

// This example syncs the account and prints the balance
async function run() {
    initLogger();
    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
            );
        }

        const wallet = new Wallet({
            storagePath: './alice-database',
        });

        const account = await wallet.getAccount('Alice');

        await account.sync();

        // To sign a transaction we need to unlock stronghold.
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);

        // Only the unspent outputs in the account
        const output_ids = await account.getOutputsWithAdditionalUnlockConditions(OutputsToClaim.All);
        console.log(`Available outputs to claim:`);
        for (const output_id of output_ids) {
            console.log(output_id);
        }

        const transaction = await account.claimOutputs(output_ids)

        console.log(`Transaction sent: ${transaction.transactionId}`);

        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );

        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
