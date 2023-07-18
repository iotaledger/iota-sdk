const path = require('path')
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const { AccountManager, CoinType } = require('@iota/wallet');

async function getUnlockedManager() {
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }
    if (!process.env.STRONGHOLD_PASSWORD) {
        throw new Error('.env STRONGHOLD_PASSWORD is undefined, see .env.example');
    }

    const manager = new AccountManager({
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: [process.env.NODE_URL],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: process.env.STRONGHOLD_SNAPSHOT_PATH,
                password: process.env.STRONGHOLD_PASSWORD,
            },
        },
    });
    return manager;
}

module.exports = getUnlockedManager;
