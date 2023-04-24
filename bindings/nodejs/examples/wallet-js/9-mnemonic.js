/**
 * This example generates, stores and verifies a mnemonic
 */

const { Utils } = require('@iota/sdk');
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();
        const mnemonic = Utils.generateMnemonic();
        console.log('Mnemonic:', mnemonic);

        Utils.verifyMnemonic(mnemonic);

        await manager.storeMnemonic(mnemonic);
        console.log('Mnemonic successfully stored!');
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit();
}

run();
