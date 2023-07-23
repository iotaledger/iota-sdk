const path = require('path');
require('dotenv').config({ path: path.resolve(__dirname, '.env') });
const {
    AccountManager,
    CoinType,
    migrateStrongholdSnapshotV2ToV3,
} = require('@iota/wallet');

const v2Path = '../../../sdk/tests/wallet/fixtures/v2.stronghold';
const v3Path = './v3.stronghold';

async function run() {
    var accountManagerOptions = {
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: [process.env.NODE_URL],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: v2Path,
                password: 'current_password',
            },
        },
    };

    try {
        // This should fail with error, migration required.
        new AccountManager(accountManagerOptions);
    } catch (error) {
        console.error(error);
    }

    migrateStrongholdSnapshotV2ToV3(
        v2Path,
        'current_password',
        'wallet.rs',
        100,
        v3Path,
        'new_password',
    );

    accountManagerOptions = {
        storagePath: process.env.WALLET_DB_PATH,
        clientOptions: {
            nodes: [process.env.NODE_URL],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: v3Path,
                password: 'new_password',
            },
        },
    };

    // This shouldn't fail anymore as snapshot has been migrated.
    new AccountManager(accountManagerOptions);

    process.exit(0);
}

run();
