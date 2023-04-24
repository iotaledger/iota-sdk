/**
 * This example generates an address without storing it.
 */
const path = require('path');
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const { Wallet, CoinType } = require('@iota/sdk');

async function run() {
    try {
        const manager = await createWallet();

        const address = await manager.generateAddress(
            0,
            false,
            0,
            { ledgerNanoPrompt: false },
            'tst',
        );
        console.log('Address:', address);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

async function createWallet() {
    const walletOptions = {
        storagePath: './alice-database',
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: `./wallet.stronghold`,
                password: `${process.env.STRONGHOLD_PASSWORD}`,
            },
        },
    };

    const manager = new Wallet(walletOptions);
    try {
        await manager.storeMnemonic(process.env.MNEMONIC);
    } catch (e) {
        console.log(e);
    }
    return manager;
}

run();
