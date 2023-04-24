const path = require('path');
require('dotenv').config({ path: '.env' });
const { Wallet, CoinType } = require('@iota/sdk');

async function getUnlockedManager() {
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error(
            '.env STRONGHOLD_PASSWORD is undefined, see .env.example',
        );
    }

    const manager = new Wallet({
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
    });
    return manager;
}

module.exports = getUnlockedManager;
