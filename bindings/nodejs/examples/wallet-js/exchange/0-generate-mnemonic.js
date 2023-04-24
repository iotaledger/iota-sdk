/**
 * This example generates a new random mnemonic
 */

const { Utils } = require('@iota/sdk');

async function run() {
    try {
        console.log('Generated mnemonic:', Utils.generateMnemonic());
        // Set generated mnemonic as env variable for MNEMONIC so it can be used in 1-create-account.js
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
