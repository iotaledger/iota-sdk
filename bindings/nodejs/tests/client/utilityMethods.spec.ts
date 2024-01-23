// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import 'reflect-metadata';
import 'dotenv/config';

import { Client, SecretManager, Utils } from '../../';
import '../customMatchers';
import { SlotCommitment } from '../../out/types/block/slot';

async function makeOfflineClient(): Promise<Client> {
    return await Client.create({});
}

describe('Client utility methods', () => {
    // Requires "stronghold" in cargo toml iota-client features
    it.skip('generates and stores mnemonic', async () => {
        const mnemonic = Utils.generateMnemonic();

        const strongholdSecretManager = {
            stronghold: {
                password: 'some_hopefully_secure_password',
                snapshotPath: './stronghold',
            },
        };
        await expect(
            SecretManager.create(strongholdSecretManager).storeMnemonic(mnemonic),
        ).resolves.toBe(null);
    });

    it('converts address to hex and bech32', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = Utils.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        let offlineClient = await makeOfflineClient();
        const bech32Address = await offlineClient.hexToBech32(
            hexAddress,
            'rms',
        );

        expect(bech32Address).toBe(address);
    });

    it('account id to address', async () => {
        const accountId =
            '0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f';

        const offlineClient = await makeOfflineClient();
        const accountAddress = await offlineClient.accountIdToBech32(
            accountId,
            'rms',
        );

        expect(accountAddress).toBe(
            'rms1pr8swlf8v6rt5exqgp9eavk324t8sggnckseshex9dc0n9jd8w7h7wcnhn7',
        );
    });

    it('sign and verify Ed25519', async () => {
        const secretManager = {
            mnemonic: Utils.generateMnemonic(),
        };

        // `IOTA` hex encoded
        const message = '0x494f5441';
        const signature = await SecretManager.create(secretManager).signEd25519(
            message,
            {
                coinType: 4218,
                account: 0,
                change: 0,
                addressIndex: 0,
            },
        );

        const validSignature = Utils.verifyEd25519Signature(signature, message);
        expect(validSignature).toBeTruthy();
    });
});
