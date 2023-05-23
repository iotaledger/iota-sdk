/**
 * This example will destroy an alias
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    // Replace with an existing alias ID in your account
    const ALIAS_ID = '0x982667c59ade8ab8a99188f4de38c68b97fc2ca7ba28a1e9d8d683996247e152'

    try {
        const manager = await getUnlockedManager();

        const account = await manager.getAccount('0');

        await account.sync();

        let tx = await account.destroyAlias(ALIAS_ID);
        console.log(tx);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
