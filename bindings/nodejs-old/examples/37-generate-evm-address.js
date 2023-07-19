/**
 * In this example we generate an EVM address.
 */

const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        let manager = await getUnlockedManager();

        const account = await manager.getAccount('Alice');

        const evmAddresses = await account.generateEvmAddresses({
            coinType: 60,
            accountIndex: 0,
        })

        console.log('Evm Address:', evmAddresses)

    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
