/**
 * This example gets the balance for an account
 */

const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();
        const account = await manager.getAccount('Alice');
        const accountAddresses = await account.addresses();
        console.log('Addresses before:', accountAddresses);

        // Always sync before calling getBalance()
        const synced = await account.sync();
        console.log('Syncing... - ', synced);

        console.log('Available balance', await account.getBalance());

        let addresses = await Promise.all(accountAddresses.map(async (a) => a.address));
        console.log('Addresses:');
        for (let address of addresses) {
            console.log(` - ${process.env.EXPLORER_URL}/addr/${address}`);
        }

        // Use the Faucet to send testnet tokens to your address:
        console.log("Fill your address with the Faucet: https://faucet.testnet.shimmer.network")
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
