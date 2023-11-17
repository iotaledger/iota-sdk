// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Balance, Wallet, CoinType, SecretManager } from '../node/lib';

async function run() {
    const mnemonicSecretManager = {
        mnemonic:
            'inhale gorilla deny three celery song category owner lottery rent author wealth penalty crawl hobby obtain glad warm early rain clutch slab august bleak',
    };

    const secretManager = new SecretManager(mnemonicSecretManager);

    const walletAddress = await secretManager.generateEd25519Addresses({
        coinType: CoinType.IOTA,
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
        bech32Hrp: 'tst',
    });

    const wallet = await Wallet.create({
        address: walletAddress[0],
        bipPath: {
            coinType: CoinType.IOTA,
        },
        clientOptions: {
            nodes: ['http://localhost:8080'],
        },
        secretManager: mnemonicSecretManager,
    });

    expect(await wallet.address()).toBe(
        'tst1qrpwecegav7eh0z363ca69laxej64rrt4e3u0rtycyuh0mam3vq3uht4pcw',
    );

    const balance: Balance = await wallet.sync();
    expect(balance.baseCoin.available).not.toBeNaN();
}

describe('wallet tests', () => {
    jest.setTimeout(10000);
    it('wallet', async () => {
        await run();
    });
});
