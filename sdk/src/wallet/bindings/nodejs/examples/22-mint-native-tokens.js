/**
 * This example mints native tokens
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();

        const account = await manager.getAccount('0');

        await account.sync();

        // First create an alias output, this needs to be done only once, because an alias can have many foundry outputs.
        let tx = await account
            .prepareCreateAliasOutput()
            .then((prepared) => prepared.finish());
        console.log('Transaction ID: ', tx.transactionId);

        // Wait for transaction inclusion
        await new Promise((resolve) => setTimeout(resolve, 5000));

        await account.sync();

        // If we omit the AccountAddress field the first address of the account is used by default
        const params = {
            // Hello in bytes
            foundryMetadata: '0x48656c6c6f',
            circulatingSupply: '0x64',
            maximumSupply: '0x64',
        };

        let response = await account
            .prepareMintNativeToken(params)
            .then((prepared) => prepared.finish());
        console.log(
            `Check your block on ${process.env.EXPLORER_URL}/block/${response.blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
