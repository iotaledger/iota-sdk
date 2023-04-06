const { AccountManager, CoinType } = require('@iota/wallet');

const NODE_URL = 'https://api.testnet.shimmer.network'
const STORAGE_PATH = 'walletdb'
const STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold'

async function main() {
    // Change to a secure password.
    let password = 'some-secure-password'

    // Set up and store the wallet.
    const accountManagerOptions = {
        storagePath: STORAGE_PATH,
        clientOptions: {
            nodes: [NODE_URL],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            stronghold: {
                snapshotPath: STRONGHOLD_SNAPSHOT_PATH,
                password: password,
            },
        },
    };

    const manager = new AccountManager(accountManagerOptions);

    // Generate a mnemonic and store it in the Stronghold vault.
    const mnemonic = await manager.generateMnemonic();
    await manager.storeMnemonic(mnemonic);

    // Create an account and get the first address.
    const account = await manager.createAccount({
        alias: 'Alice',
    });
    const address = await account.addresses().then(addresses => addresses[0]);

    // Print the account data.
    console.log(`Mnemonic:\n${mnemonic}\n`);
    console.log(`Address:\n${address.address}\n`);

    process.exit(0);
}

main();
