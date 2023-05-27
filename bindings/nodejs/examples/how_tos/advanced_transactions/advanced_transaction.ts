// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    Client,
    Ed25519Address,
    TimelockUnlockCondition,
    Utils,
    Wallet,
    initLogger,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/advanced_transactions/advanced_transaction.ts

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

        //TODO: Replace with the address of your choice!
        const address =
            'rms1qrrv7flg6lz5cssvzv2lsdt8c673khad060l4quev6q09tkm9mgtupgf0h0';
        const amount = '1000000';

        // Create an ouput with amount 1_000_000 and a timelock of 1 hour
        const in_an_hour = Math.floor(Date.now() / 1000) + 3600;
        const basicOutput = await new Client({}).buildBasicOutput({
            unlockConditions: [
                new AddressUnlockCondition(
                    new Ed25519Address(
                        Utils.bech32ToHex(
                            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
                        ),
                    ),
                ),
                new TimelockUnlockCondition(in_an_hour),
            ],
        });

        const transaction = await account.sendOutputs([basicOutput]);
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
