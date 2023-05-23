// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    BasicOutputBuilderParams,
    Ed25519Address,
    SendNativeTokensParams,
} from '@iota/sdk';

import { getUnlockedManager } from './account-manager';

// The native token amount to send, `10` hex encoded
const SEND_NATIVE_TOKEN_AMOUNT = '0xA';
// The address to send the tokens to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send an amount below the minimum storage deposit.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `01-create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/07-send-native-tokens.ts
async function run() {
    try {
        // Create the wallet
        const manager = await getUnlockedManager();

        // Get the account we generated with `01-create-wallet`
        const account = await manager.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        let balance = await account.sync();

        // Get a token with sufficient balance
        // TODO: use BigNumber library
        let tokenId = balance.nativeTokens.find(
            (t) => Number(t.available) >= Number(SEND_NATIVE_TOKEN_AMOUNT),
        )?.tokenId;

        if (tokenId != null) {
            let outputs: SendNativeTokensParams[] = [
                {
                    address: RECV_ADDRESS,
                    nativeTokens: [[tokenId, SEND_NATIVE_TOKEN_AMOUNT]],
                },
            ];

            console.log(
                `Sending '${Number(
                    SEND_NATIVE_TOKEN_AMOUNT,
                )}' coin(s) to '${RECV_ADDRESS}'...`,
            );

            let transaction = await account
                .prepareSendNativeTokens(outputs)
                .then((prepared) => prepared.finish());

            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            let blockId = await account.retryTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            await account.sync();
            console.log('Account synced');

            console.log('Sending basic output transaction...');

            // Send native tokens together with the required storage deposit
            let client = await manager.getClient();
            let rentStructure = await client
                .getInfo()
                .then((info) => info.nodeInfo.protocol.rentStructure);

            // TODO: build from rent structure
            let basicOutput: BasicOutputBuilderParams = {
                amount: '1',
                unlockConditions: [
                    new AddressUnlockCondition(
                        new Ed25519Address(RECV_ADDRESS),
                    ),
                ],
                nativeTokens: [
                    {
                        id: tokenId,
                        amount: SEND_NATIVE_TOKEN_AMOUNT,
                    },
                ],
            };

            let output = await client.buildBasicOutput(basicOutput);
            transaction = await account.sendOutputs([output]);

            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            blockId = await account.retryTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Transaction included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
