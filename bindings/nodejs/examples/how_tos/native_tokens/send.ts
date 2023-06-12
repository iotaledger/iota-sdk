// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    SendNativeTokensParams,
} from '@iota/sdk';

import { getUnlockedWallet } from '../../wallet/common';

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
        const account = await wallet.getAccount('Alice');

        // May want to ensure the account is synced before sending a transaction.
        var balance = await account.sync();

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

            let token = balance.nativeTokens.find(
                (nativeToken) => nativeToken.tokenId == tokenId,
            );
            if (token == null) {
                throw new Error(
                    `Couldn't find native token '${tokenId}' in the account`,
                );
            }
            console.log(`Balance before sending:`, parseInt(token.available));

            let transaction = await account
                .prepareSendNativeTokens(outputs)
                .then((prepared) => prepared.send());

            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            let blockId = await account.retryTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            balance = await account.sync();
            token = balance.nativeTokens.find(
                (nativeToken) => nativeToken.tokenId == tokenId,
            );
            if (token == null) {
                throw new Error(
                    `Couldn't find native token '${tokenId}' in the account`,
                );
            }
            console.log(`Balance after sending:`, parseInt(token.available));
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
