/**
 * This example generates an address for an account
 */

require('dotenv').config();
const { AccountManager } = require('@iota/wallet');

async function run() {
    try {
        const manager = new AccountManager({
            storagePath: process.env.WALLET_DB_PATH,
        });

        await manager.setStrongholdPassword(
            process.env.STRONGHOLD_PASSWORD,
        );

        const account = await manager.getAccount('Alice');

        const address = await account.generateEd25519Address();

        console.log('Address generated:', address);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
