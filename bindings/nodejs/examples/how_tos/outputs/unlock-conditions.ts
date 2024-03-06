// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    UnlockCondition,
    AddressUnlockCondition,
    StorageDepositReturnUnlockCondition,
    ExpirationUnlockCondition,
    TimelockUnlockCondition,
    SimpleTokenScheme,
    ImmutableAccountAddressUnlockCondition,
    AccountAddress,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/unlock-conditions.ts

// Build outputs with all unlock conditions
async function run() {
    initLogger();

    const client = await Client.create({});

    try {
        const ed25519Address = Utils.parseBech32Address(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const accountAddress = Utils.parseBech32Address(
            'rms1pr59qm43mjtvhcajfmupqf23x29llam88yecn6pyul80rx099krmv2fnnux',
        ) as AccountAddress;

        const tokenSchema = new SimpleTokenScheme(
            BigInt(50),
            BigInt(0),
            BigInt(100),
        );

        const addressUnlockCondition: UnlockCondition =
            new AddressUnlockCondition(ed25519Address);

        // Most simple output
        const basicOutput = await client.buildBasicOutput({
            unlockConditions: [addressUnlockCondition],
        });

        // Output with storage deposit return
        const basicOutputWithStorageReturn = await client.buildBasicOutput({
            unlockConditions: [
                addressUnlockCondition,
                new StorageDepositReturnUnlockCondition(
                    ed25519Address,
                    '1000000',
                ),
            ],
        });

        // Output with timelock
        const basicOutputWithTimelock = await client.buildBasicOutput({
            unlockConditions: [
                addressUnlockCondition,
                new TimelockUnlockCondition(1),
            ],
        });

        // Output with expiration
        const basicOutputWithExpiration = await client.buildBasicOutput({
            unlockConditions: [
                addressUnlockCondition,
                new ExpirationUnlockCondition(ed25519Address, 1),
            ],
        });

        // Output with immutable account unlock condition
        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 1,
            tokenScheme: tokenSchema,
            unlockConditions: [
                new ImmutableAccountAddressUnlockCondition(accountAddress),
            ],
        });

        console.log(
            JSON.stringify(
                [
                    basicOutput,
                    basicOutputWithStorageReturn,
                    basicOutputWithTimelock,
                    basicOutputWithExpiration,
                    foundryOutput,
                ],
                null,
                2,
            ),
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
