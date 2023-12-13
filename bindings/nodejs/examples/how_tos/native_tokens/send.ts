// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SendNativeTokenParams } from '@iota/sdk';

import { getUnlockedWallet } from '../../wallet/common';

// The native token amount to send.
const SEND_NATIVE_TOKEN_AMOUNT = BigInt(10);
// The address to send the token to
const RECV_ADDRESS =
    'rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu';

// In this example we will send a native token.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./how_tos/native_tokens/send.ts
async function run() {
    for (const envVar of ['EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        // May want to ensure the wallet is synced before sending a transaction.
        let balance = await wallet.sync();

        // Get a token with sufficient balance
        const tokenId = balance.nativeTokens.find(
            (t) => Number(t.available) >= Number(SEND_NATIVE_TOKEN_AMOUNT),
        )?.tokenId;

        if (tokenId != null) {
            const outputs: SendNativeTokenParams[] = [
                {
                    address: RECV_ADDRESS,
                    nativeToken: [tokenId, SEND_NATIVE_TOKEN_AMOUNT],
                },
            ];

            let token = balance.nativeTokens.find(
                (nativeToken) => nativeToken.tokenId == tokenId,
            );
            if (token == null) {
                throw new Error(
                    `Couldn't find native token '${tokenId}' in the wallet`,
                );
            }
            console.log(`Balance before sending: ${token.available}`);

            const transaction = await wallet.sendNativeTokens(outputs);

            console.log(`Transaction sent: ${transaction.transactionId}`);

            // Wait for transaction to get included
            const blockId = await wallet.reissueTransactionUntilIncluded(
                transaction.transactionId,
            );

            console.log(
                `Block included: ${process.env.EXPLORER_URL}/block/${blockId}`,
            );

            balance = await wallet.sync();
            token = balance.nativeTokens.find(
                (nativeToken) => nativeToken.tokenId == tokenId,
            );
            if (token == null) {
                throw new Error(
                    `Couldn't find native token '${tokenId}' in the wallet`,
                );
            }
            console.log(`Balance after sending: ${token.available}`);
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
