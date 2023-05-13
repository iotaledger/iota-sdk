/**
 * This example will mint an NFT
 */
const getUnlockedManager = require('./account-manager');

async function run() {
    try {
        const manager = await getUnlockedManager();

        const account = await manager.getAccount('0');

        await account.sync();

        const response = await account.prepareMintNfts([
            {
                // Hello in bytes
                immutableMetadata: '0x48656c6c6f',
                metadata: '0x48656c6c6f',
            }
        ]).then(prepared => prepared.finish());

        console.log(response);

        console.log(
            `Check your block on ${process.env.EXPLORER_URL}/block/${response.blockId}`,
        );
    } catch (error) {
        console.log('Error: ', error);
    }
    process.exit(0);
}

run();
