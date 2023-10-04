// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    UnlockCondition,
    Ed25519Address,
    SenderFeature,
    MetadataFeature,
    Wallet,
    AddressUnlockCondition,
    AliasAddress,
} from '@iota/sdk';
import {
    prepareMetadata
} from './utils';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/evm/send_base_token_to_evm.ts

const amount = 1000000;
const gas = 10000;
const toEVMAddress = '0x48e28C1681BBb92a2E5874113bc740cC11A0FD7a';
const chainAddress = 'rms1pr75wa5xuepg2hew44vnr28wz5h6n6x99zptk2g68sp2wuu2karywgrztx3';

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

    // After syncing the balance can also be computed with the local data
    const balance = await account.getBalance();
    console.log('Balance::', balance.baseCoin);

    try {
        const addresses = await account.addresses();
        const hexAddress = Utils.bech32ToHex(
            addresses[0].address,
        );

        const aliasHexAddress = Utils.bech32ToHex(
            chainAddress,
        );
        const addressUnlockCondition: UnlockCondition = new AddressUnlockCondition(new AliasAddress(aliasHexAddress))

        const addressFeature = new SenderFeature(new Ed25519Address(hexAddress));

        const metadata = await prepareMetadata(
            toEVMAddress,
            BigInt(amount),
            BigInt(gas)
        );
        const metadataFeature = new MetadataFeature(metadata);

        // Basic Output with Metadata
        const basicOutput = await client.buildBasicOutput({
            amount: amount.toString(),
            unlockConditions: [
                addressUnlockCondition
            ],
            features: [
                addressFeature,
                metadataFeature,
            ],
        });
        console.log('basicOutput:', JSON.stringify(basicOutput, null, 2));

        // Send Output
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);
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
