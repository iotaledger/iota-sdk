import {
    Wallet,
    CoinType,
    WalletOptions,
    Utils,
    SecretManager,
} from '@iota/sdk';

async function run() {

    const STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold';

    const secretManager = SecretManager.create({
        stronghold: {
            snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
            password: "hello"
        },
    });
    const mnemonic = Utils.generateMnemonic();
    await secretManager.storeMnemonic(mnemonic);

    const walletOptions: WalletOptions = {
        storagePath: 'getting-started-db',
        clientOptions: {
            nodes: ['https://api.testnet.shimmer.network'],
        },
        bipPath: {
            coinType: CoinType.IOTA,
        },
        secretManager: {
            stronghold: {
                snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
            },
        },
    };

    const wallet = await Wallet.create(walletOptions);

    console.log('Created', wallet);
}

void run().then(() => process.exit());
