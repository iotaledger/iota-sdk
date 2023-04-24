/**
 * This example creates a new database and account
 */

require('dotenv').config();
const { Wallet, CoinType } = require('@iota/sdk');

async function run() {
    try {
        const walletOptions = {
            storagePath: './alice-database',
            clientOptions: {
                nodes: ['https://api.testnet.shimmer.network'],
            },
            // CoinType.IOTA can be used to access Shimmer staking rewards, but it's
            // recommended to use the Shimmer coin type to be compatible with other wallets.
            coinType: CoinType.Shimmer,
            secretManager: {
                Stronghold: {
                    snapshotPath: `./wallet.stronghold`,
                    password: `${process.env.STRONGHOLD_PASSWORD}`,
                },
            },
        };

        const manager = new Wallet(walletOptions);

        // Mnemonic only needs to be set the first time
        await manager.storeMnemonic(process.env.MNEMONIC);

        const account = await manager.createAccount({
            alias: 'Alice',
        });
        console.log('Account created:', account);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
