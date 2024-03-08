// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { describe, it, expect } from '@jest/globals';
import {
    CoinType,
    SecretManager,
    Utils,
} from '../../lib/';

describe('SecretManager', () => {
    it('generate IOTA Ed25519 address', async () => {
        const mnemonicSecretManager = {
            mnemonic: "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"
        };

        let bech32_hrp = Utils.iotaMainnetProtocolParameters().bech32Hrp;

        const secretManager = SecretManager.create(mnemonicSecretManager);
        const addresses = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            bech32Hrp: bech32_hrp,
        });

        expect(addresses[0]).toEqual('iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg');

    }, 20000);

    it('generate Shimmer Ed25519 address', async () => {
        const mnemonicSecretManager = {
            mnemonic: "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"
        };

        let bech32_hrp = Utils.shimmerMainnetProtocolParameters().bech32Hrp;

        const secretManager = SecretManager.create(mnemonicSecretManager);
        const addresses = await secretManager.generateEd25519Addresses({
            coinType: CoinType.Shimmer,
            bech32Hrp: bech32_hrp,
            range: { start: 0, end: 1 },
        });

        expect(addresses[0]).toEqual('smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y');

    }, 20000);
});