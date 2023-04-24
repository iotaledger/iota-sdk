/**
 * This example generates an address for an account
 */

require('dotenv').config();
const { Wallet } = require('@iota/sdk');

async function run() {
    try {
        const manager = new Wallet({
            storagePath: './alice-database',
        });

        await manager.setStrongholdPassword(
            `${process.env.STRONGHOLD_PASSWORD}`,
        );

        const account = await manager.getAccount('Alice');

        const address = await account.generateAddress();

        console.log('Address generated:', address);
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
