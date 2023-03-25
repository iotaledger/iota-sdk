/**
 * This example will destroy an alias
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    // Replace with an existing alias ID in your account
    const ALIAS_ID = '0x08e6210d29881310db2afde095e594f6f006fcdbd06e7a83b74bd2bdf3b5190d0e0200000000'
    
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
