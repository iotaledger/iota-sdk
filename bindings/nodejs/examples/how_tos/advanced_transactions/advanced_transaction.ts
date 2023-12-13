// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    Ed25519Address,
    TimelockUnlockCondition,
    Utils,
    Wallet,
    initLogger,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/advanced_transaction.ts

// This example syncs the account and sends an output with a timelock unlock condition
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

        const client = await wallet.getClient();

        // Create an output with amount 1_000_000 and a timelock of 1 hour
        // TODO Make the slot index properly 1h ahead
        const slotIndex = 1000;
        const basicOutput = await client.buildBasicOutput({
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(
                        Utils.bech32ToHex(
                            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
                        ),
                    ),
                ),
                new TimelockUnlockCondition(slotIndex),
            ],
        });

        const transaction = await wallet.sendOutputs([basicOutput]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        console.log('Waiting until included in block...');
        const blockId = await wallet.reissueTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
