// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AddressUnlockCondition,
    BasicOutputBuilderParams,
    Ed25519Address,
    SendNativeTokensParams,
} from '@iota/sdk';

import { getUnlockedWallet } from './common';

// The native token amount to send, `10` hex encoded
const SEND_NATIVE_TOKEN_AMOUNT = '0xA';
// The address to send the tokens to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send native tokens.
//
// Make sure that `example.stronghold` and `example.walletdb` already exist by
// running the `how_tos/accounts-and-addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/07-send-native-tokens.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // Get the account we generated with `01-create-wallet`
        const account = await wallet.getAccount(
            `${process.env.ACCOUNT_ALIAS_1}`,
        );

        // May want to ensure the account is synced before sending a transaction.
        const balance = await account.sync();

        // Get a token with sufficient balance
        // TODO: use BigNumber library
        const tokenId = balance.nativeTokens.find(
            (t) => Number(t.available) >= Number(SEND_NATIVE_TOKEN_AMOUNT),
        )?.tokenId;

        if (tokenId != null) {
            const outputs: SendNativeTokensParams[] = [
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
                .then((prepared) => prepared.send());

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
            const client = await wallet.getClient();

            const basicOutput: BasicOutputBuilderParams = {
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

            const output = await client.buildBasicOutput(basicOutput);
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
