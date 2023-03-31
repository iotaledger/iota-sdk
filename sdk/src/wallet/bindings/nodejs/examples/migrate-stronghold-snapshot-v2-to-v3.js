const path = require('path')
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const { AccountManager, CoinType, migrateStrongholdSnapshotV2ToV3 } = require('@iota/wallet');

var accountManagerOptions = {
    storagePath: './alice-database',
    clientOptions: {
        nodes: [process.env.NODE_URL],
        localPow: true,
    },
    coinType: CoinType.Shimmer,
    secretManager: {
        Stronghold: {
            snapshotPath: "fixtures/v2.stronghold",
            password: "current_password",
        },
    },
};

try {
    new AccountManager(accountManagerOptions);
} catch (error) {
    console.error(error);
}

migrateStrongholdSnapshotV2ToV3("fixtures/v2.stronghold", "current_password", "fixtures/v3.stronghold", "new_password")

accountManagerOptions = {
    storagePath: './alice-database',
    clientOptions: {
        nodes: [process.env.NODE_URL],
        localPow: true,
    },
    coinType: CoinType.Shimmer,
    secretManager: {
        Stronghold: {
            snapshotPath: "fixtures/v3.stronghold",
            password: "new_password",
        },
    },
};

const manager = new AccountManager(accountManagerOptions);