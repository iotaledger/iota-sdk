// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { describe, it, expect } from '@jest/globals';
import { Wallet, CoinType, WalletOptions, SecretManager } from '../../lib/';

describe('Wallet', () => {
    it('create wallet', async () => {
        let storagePath = 'test-create-wallet';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        };

        const secretManager = new SecretManager(strongholdSecretManager);

        await secretManager.storeMnemonic('vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',);

        const wallet_address = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'tst',
        });

        const walletOptions: WalletOptions = {
            address: wallet_address[0],
            storagePath: './test-create-wallet',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: {
                coinType: CoinType.IOTA,
            },
            secretManager: strongholdSecretManager,
        };


        const wallet = new Wallet(walletOptions);

        await wallet.destroy()
        removeDir(storagePath)
    }, 8000);


    it('recreate wallet', async () => {
        let storagePath = 'test-recreate-wallet';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        };

        const secretManager = new SecretManager(strongholdSecretManager);

        await secretManager.storeMnemonic('vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',);

        const wallet_address = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'tst',
        });

        const walletOptions: WalletOptions = {
            address: wallet_address[0],
            storagePath: './test-recreate-wallet',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: {
                coinType: CoinType.IOTA,
            },
            secretManager: strongholdSecretManager,
        };


        const wallet = new Wallet(walletOptions);

        const client = await wallet.getClient();
        const hrp = await client.getBech32Hrp();
        expect(hrp).toEqual("smr");

        await wallet.destroy();

        const recreatedWallet = new Wallet({ storagePath: './test-recreate-wallet' });

        await recreatedWallet.destroy()
        removeDir(storagePath)
    }, 20000);
})

function removeDir(storagePath: string) {
    const fs = require('fs');
    fs.rmSync(storagePath, { recursive: true, force: true });
}
