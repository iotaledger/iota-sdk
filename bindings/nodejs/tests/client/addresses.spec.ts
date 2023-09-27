// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import { SecretManager } from '../../out';
import '../customMatchers';

import * as mnemonicAddressTestCases from '../../../../sdk/tests/client/fixtures/test_vectors.json';

const secretManager = {
    mnemonic:
        'endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river',
};

describe('Address tests', () => {

    it('calculates addresses according a fixture', async () => {

        for (const test of mnemonicAddressTestCases.general.address_generations) {
            const secretManager = await new SecretManager({
                mnemonic: test['mnemonic']
            });
        
            const generatedAddress = await secretManager.generateEd25519Addresses({
                coinType: test['coin_type'],
                accountIndex: test['account_index'],
                range: {
                    start: test['address_index'],
                    end: test['address_index'] + 1,
                },
                bech32Hrp: test['bech32_hrp'],
                options: {
                    internal: test['internal'],
                }
            });
        
            if (test['bech32_address'] !== generatedAddress[0]) {
              throw new Error('Test failed: Bech32 address does not match generated address.');
            }
          }
    });

    it('generates addresses', async () => {
        const addresses = await new SecretManager(secretManager).generateEd25519Addresses({
            accountIndex: 0,
            range: {
                start: 0,
                end: 5,
            },
            bech32Hrp: 'rms',
        });

        expect(addresses.length).toBe(5);

        addresses.forEach((address) => {
            expect(address).toBeValidAddress();
        });
    });
});
