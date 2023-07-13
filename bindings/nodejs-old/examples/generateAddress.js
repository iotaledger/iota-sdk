/**
 * This example generates an address without storing it.
 */
const path = require('path');
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const { AccountManager, CoinType } = require('@iota/wallet');

async function run() {
    try {
        const manager = await createAccountManager();

        const address = await manager.generateEd25519Address(
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

async function createAccountManager() {
    const accountManagerOptions = {
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                password: process.env.STRONGHOLD_PASSWORD,
            },
        },
    };

    const manager = new AccountManager(accountManagerOptions);
    try {
        await manager.storeMnemonic(process.env.MNEMONIC);
    } catch (e) {
        console.log(e);
    }
    return manager;
}

run();
