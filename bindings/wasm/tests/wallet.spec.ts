// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, CoinType, SecretManager } from '../node/lib';

describe('wallet tests', () => {
    jest.setTimeout(100000);
    it('wallet', async () => {
        const mnemonicSecretManager = {
            mnemonic:
                'inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak',
        };

        const secretManager = SecretManager.create(mnemonicSecretManager);

        const walletAddress = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'rms',
        });

        const wallet = await Wallet.create({
            address: walletAddress[0],
            bipPath: {
                coinType: CoinType.IOTA,
            },
            clientOptions: {
                nodes: ['http://localhost:8050'],
            },
            secretManager: mnemonicSecretManager,
        });

        const implicitAccountCreationAddress =
            await wallet.implicitAccountCreationAddress();

        expect(implicitAccountCreationAddress).toBe(
            'rms1yrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3uvx3u3m',
        );
    });
});
