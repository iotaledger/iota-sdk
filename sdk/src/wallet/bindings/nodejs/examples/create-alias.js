/**
 * This example creates an alias output
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();

        const account = await manager.getAccount('0');

        await account.sync();

        // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs.
        let tx = await account.createAliasOutput()
        console.log(tx);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
