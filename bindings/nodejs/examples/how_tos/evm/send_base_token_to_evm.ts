// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    UnlockCondition,
    AddressUnlockCondition,
    Ed25519Address,
    SenderFeature,
    MetadataFeature,
    Wallet,
    TransactionOptions,
} from '@iota/sdk';
import {
    prepareMetadata
} from './utils';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/unlock-conditions.ts

// Build ouputs with all unlock conditions
async function run() {
    initLogger();

    const client = new Client({});

    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }

    const wallet = new Wallet({
        storagePath: process.env.WALLET_DB_PATH,
    });

    const account = await wallet.getAccount('Alice');

    // Sync new outputs from the node.
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const _syncBalance = await account.sync();

    // After syncing the balance can also be computed with the local data
    const balance = await account.getBalance();
    console.log('Balance::', balance.baseCoin);

    const amount = 100000;
    const gas = 10000;

    const bigAmount: bigint = BigInt(amount);
    const bigGas: bigint = BigInt(gas);
    // console.log('amounts:', bigAmount, bigGas);

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const addressUnlockCondition: UnlockCondition = new AddressUnlockCondition(new Ed25519Address(hexAddress));

        const addressFeature = new SenderFeature(new Ed25519Address(hexAddress));
        // console.log('addressFeature:', addressFeature);

        const metadata = await prepareMetadata(
            '0x48e28C1681BBb92a2E5874113bc740cC11A0FD7a',
            bigAmount,
            bigGas
        );
        // console.log('metadata:', metadata);
        const metadataFeature = new MetadataFeature(metadata);
        // console.log('metadataFeature:', metadataFeature);

        // Basic Output with Metadata
        const basicOutput = await client.buildBasicOutput({
            amount: amount.toString(),
            unlockConditions: [addressUnlockCondition],
            features: [
                addressFeature,
                metadataFeature,
              ],
        });
        console.log('basicOutput:', JSON.stringify(basicOutput, null, 2));

        // let transactionOptions: TransactionOptions;

        // Allowance

        // Send Output
        console.log('Sending Transaction...');
        const transaction = await account.sendOutputs([basicOutput]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        console.log('Waiting until included in block...');
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
