// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AddressUnlockCondition, BasicOutput, Output, Utils } from '@iota/sdk';

import { getUnlockedWallet } from './common';

// The amount to build the basic output with
const AMOUNT = BigInt(1000000);

// In this example we check if an output has only an address unlock condition and that the address is from the wallet.
//
// Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
// running the `how_tos/accounts_and_addresses/create-wallet` example!
//
// Rename `.env.example` to `.env` first, then run
// yarn run-example ./wallet/17-check-unlock-conditions.ts
async function run() {
    try {
        // Create the wallet
        const wallet = await getUnlockedWallet();

        const walletAddress = await wallet.address();

        const output: Output = await wallet.prepareOutput({
            recipientAddress: walletAddress,
            amount: AMOUNT,
        });

        const hexEncodedwalletAddress = Utils.bech32ToHex(walletAddress);

        if (output instanceof BasicOutput) {
            const basicOutput = output as BasicOutput;
            let controlledByWallet = false;
            if (
                basicOutput.unlockConditions.length === 1 &&
                basicOutput.unlockConditions[0] instanceof
                    AddressUnlockCondition &&
                hexEncodedwalletAddress.includes(
                    (
                        basicOutput
                            .unlockConditions[0] as AddressUnlockCondition
                    ).address.toString(),
                )
            ) {
                controlledByWallet = true;
            }

            console.log(
                'The output has only an address unlock condition and the address is from the wallet: ' +
                    controlledByWallet,
            );
        }
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

void run().then(() => process.exit());
