/**
 * This example creates a new database and account
 */
const path = require('path');
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const { Wallet, CoinType } = require('@iota/sdk');

async function run() {
    try {
        const manager = await createWallet();

        const account = await manager.createAccount({
            alias: 'Alice',
        });
        console.log('Account created:', account);

        const secondAccount = await manager.createAccount({
            alias: 'Bob',
        });
        console.log('Account created:', secondAccount);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

async function createWallet() {
    const walletOptions = {
        storagePath: './alice-database',
        clientOptions: {
            nodes: [process.env.NODE_URL],
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
    await manager.storeMnemonic(process.env.MNEMONIC);
    return manager;
}

run();
