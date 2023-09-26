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
import { SimpleBufferCursor } from './simple-buffer-cursor';

require('dotenv').config({ path: '.env' });

function hexToBytes(hex: any) {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return Buffer.from(bytes);
}

async function prepareMetadata(evmAddress: string, amount: bigint, gas: bigint) {
    const metadata = new SimpleBufferCursor();

    /* Write contract meta data */
    metadata.writeUInt8(0); // nil sender contract
    metadata.writeUInt32LE(0x3c4b5e02); // "accounts"
    metadata.writeUInt32LE(0x23f4e3a1); // "transferAllowanceTo"
    metadata.writeUInt64SpecialEncoding(gas); // gas

    /* Create evm address buffer */
    const evmAddressBuffer = new SimpleBufferCursor();
    evmAddressBuffer.writeInt8(3); // EVM address type (3)
    evmAddressBuffer.writeBytes(hexToBytes(evmAddress.toLowerCase())) // EVM address

    /* Write length of contract arguments (1) */
    metadata.writeUInt32SpecialEncoding(1);

    // Write evm address (arg1)
    metadata.writeUInt32SpecialEncoding(1);// Length of key (len(a) == 1)
    metadata.writeInt8('a'.charCodeAt(0)); // Write key (a == 'agentID')
    metadata.writeUInt32SpecialEncoding(evmAddressBuffer.buffer.length); // Length of value (len(agentID) == 21 for evm address)
    metadata.writeBytes(evmAddressBuffer.buffer); //  Write value (bytes(agentID))

    /* Write allowance */
    // see https://github.com/iotaledger/wasp/blob/12845adea4fc097813a30a061853af4a43407d3c/packages/isc/assets.go#L348-L356 
    metadata.writeUInt8(128); // 0x80 flag meaning there are native tokens in the allowance
    metadata.writeUInt64SpecialEncoding(amount - gas); // IOTA amount to send
    // console.log(metadata.buffer.toString('hex'))
    return '0x' + metadata.buffer.toString('hex');
}

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

    const amount = 51000;
    const gas = 10;

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
