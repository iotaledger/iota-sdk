// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    initLogger,
    Utils,
    UnlockCondition,
    AddressUnlockCondition,
    StorageDepositReturnUnlockCondition,
    Ed25519Address,
    ExpirationUnlockCondition,
    TimelockUnlockCondition,
    SimpleTokenScheme,
    StateControllerAddressUnlockCondition,
    GovernorAddressUnlockCondition,
    ImmutableAliasAddressUnlockCondition,
    AliasAddress,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/outputs/unlock-conditions.ts

// Build outputs with all unlock conditions
async function run() {
    initLogger();

    const client = new Client({});

    try {
        const hexAddress = Utils.bech32ToHex(
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
        );

        const aliasHexAddress = Utils.bech32ToHex(
            'rms1pr59qm43mjtvhcajfmupqf23x29llam88yecn6pyul80rx099krmv2fnnux',
        );

        const tokenSchema = new SimpleTokenScheme(
            BigInt(50),
            BigInt(0),
            BigInt(100),
        );

        const addressUnlockCondition: UnlockCondition =
            new AddressUnlockCondition(new Ed25519Address(hexAddress));

        // Most simple output
        const basicOutput = await client.buildBasicOutput({
            unlockConditions: [addressUnlockCondition],
        });

        // Output with storage deposit return
        const basicOutputWithStorageReturn = await client.buildBasicOutput({
            unlockConditions: [
                addressUnlockCondition,
                new StorageDepositReturnUnlockCondition(
                    new Ed25519Address(hexAddress),
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
                new ExpirationUnlockCondition(
                    new Ed25519Address(hexAddress),
                    1,
                ),
            ],
        });

        // Output with governor and state controller unlock condition
        const aliasOutput = await client.buildAliasOutput({
            aliasId:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
            unlockConditions: [
                new GovernorAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
                new StateControllerAddressUnlockCondition(
                    new Ed25519Address(hexAddress),
                ),
            ],
        });

        // Output with immutable alias unlock condition
        const foundryOutput = await client.buildFoundryOutput({
            serialNumber: 1,
            tokenScheme: tokenSchema,
            unlockConditions: [
                new ImmutableAliasAddressUnlockCondition(
                    new AliasAddress(aliasHexAddress),
                ),
            ],
        });

        console.log(
            JSON.stringify(
                [
                    basicOutput,
                    basicOutputWithStorageReturn,
                    basicOutputWithTimelock,
                    basicOutputWithExpiration,
                    aliasOutput,
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

run().then(() => process.exit());
