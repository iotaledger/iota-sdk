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
    NftOutput,
} from '@iota/sdk';
import {
    prepareMetadata
} from './utils';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/unlock-conditions.ts

const amount = 100000;
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

    // Sync new outputs from the node.
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const _syncBalance = await account.sync();

    // After syncing the balance can also be computed with the local data
    const balance = await account.getBalance();
    console.log('nft balance:', balance.nfts.length);
    const nftId = balance.nfts[0]; // First NFT Available

    const bigAmount: bigint = BigInt(amount);
    const bigGas: bigint = BigInt(gas);
    // console.log('amounts:', bigAmount, bigGas);

    try {
        const addresses = await account.addresses();
        // console.log('address selected:', addresses[0].address);
        const hexAddress = Utils.bech32ToHex(
            addresses[0].address,
        );

        const aliasHexAddress = Utils.bech32ToHex(
            chainAddress,
        );
        const addressUnlockCondition: UnlockCondition = new AddressUnlockCondition(new AliasAddress(aliasHexAddress))

        const addressFeature = new SenderFeature(new Ed25519Address(hexAddress));
        // console.log('addressFeature:', addressFeature);

        const metadata = await prepareMetadata(
            toEVMAddress,
            bigAmount,
            bigGas
        );
        // console.log('metadata:', metadata);
        const metadataFeature = new MetadataFeature(metadata);
        // console.log('metadataFeature:', metadataFeature);

        const nftOutputData = (
            await account.unspentOutputs({ nftIds: [nftId] })
        )[0];
        // console.log(
        //     `NFT ${nftId} found in unspent output: ${nftOutputData.outputId}`,
        // );

        const nftOutput = nftOutputData.output as NftOutput;
        // console.log('nftOutput:', nftOutput);

        // Basic Output with Metadata
        const nftOutputSend = await client.buildNftOutput({
            amount: amount.toString(),
            nftId: nftId,
            unlockConditions: [
                addressUnlockCondition
            ],
            features: [
                addressFeature,
                metadataFeature,
            ],
            immutableFeatures: nftOutput.immutableFeatures
        });
        // console.log('nftOutputSend:', JSON.stringify(nftOutputSend, null, 2));

        // // Send Output
        await wallet.setStrongholdPassword(process.env.STRONGHOLD_PASSWORD);
        console.log('Sending Transaction...');
        const transaction = await account.sendOutputs([nftOutput]);
        console.log(`Transaction sent: ${transaction.transactionId}`);

        console.log('Waiting until included in block...');
        const blockId = await account.retryTransactionUntilIncluded(
            transaction.transactionId,
        );
        console.log(`Block sent: ${process.env.EXPLORER_URL}/block/${blockId}`);

        // Right now this token even though sent to L2 evm won't really show up anywhere.
        // The foundry owner needs to register it and then the ERC20 wrapper would work only for foundry owner.
        // Ref: https://wiki.iota.org/wasp-evm/reference/core-contracts/evm#registererc20nativetoken
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
