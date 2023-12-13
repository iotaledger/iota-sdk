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

        const secretManager = SecretManager.create(strongholdSecretManager);

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


        const wallet = await Wallet.create(walletOptions);

        await wallet.destroy();
        removeDir(storagePath);
    }, 20000);


    it('recreate wallet', async () => {
        let storagePath = 'test-recreate-wallet';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        };

        const secretManager = SecretManager.create(strongholdSecretManager);

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
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: {
                coinType: CoinType.IOTA,
            },
            secretManager: strongholdSecretManager,
        };


        const wallet = await Wallet.create(walletOptions);

        const client = await wallet.getClient();
        const hrp = await client.getBech32Hrp();
        expect(hrp).toEqual("smr");

        await wallet.destroy();

        const recreatedWallet = await Wallet.create({ storagePath });

        // TODO: make add test to verify wallet equality
        // expect(accounts.length).toStrictEqual(0);

        await recreatedWallet.destroy();
        removeDir(storagePath)
    }, 20000);

    it('error after destroy', async () => {
        let storagePath = 'test-error-after-destroy';
        removeDir(storagePath);

        const strongholdSecretManager = {
            stronghold: {
                snapshotPath: `./${storagePath}/wallet.stronghold`,
                password: `A12345678*`,
            },
        }
        const secretManager = await SecretManager.create(strongholdSecretManager);
        await secretManager.storeMnemonic(
            'vital give early extra blind skin eight discover scissors there globe deal goat fat load robot return rate fragile recycle select live ordinary claim',
        );

        const wallet_address = await secretManager.generateEd25519Addresses({
            coinType: CoinType.IOTA,
            accountIndex: 0,
            range: {
                start: 0,
                end: 1,
            },
            bech32Hrp: 'tst',
        });

        const walletOptions = {
            address: wallet_address[0],
            storagePath,
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            bipPath: {
                coinType: CoinType.IOTA,
            },
            secretManager: strongholdSecretManager,
        };

        const wallet = await Wallet.create(walletOptions);

        await wallet.destroy();

        try {
            const _accounts = await wallet.accounts();
            throw 'Should return an error because the wallet was destroyed';
        } catch (err: any) {
            expect(err.message).toEqual('Wallet was destroyed');
        }

        try {
            const _client = await wallet.getClient();
            throw 'Should return an error because the wallet was destroyed';
        } catch (err: any) {
            expect(err.message).toEqual('Wallet was destroyed');
        }
        removeDir(storagePath);
    }, 35000);
})

function removeDir(storagePath: string) {
    const fs = require('fs');
    fs.rmSync(storagePath, { recursive: true, force: true });
}
