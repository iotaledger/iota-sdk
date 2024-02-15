import { Utils } from '../node/lib';

describe('Utils methods', () => {
    it('generates and validates mnemonic', async () => {
        const mnemonic = Utils.generateMnemonic();

        // A mnemonic has 24 words
        expect(mnemonic.split(' ').length).toEqual(24);

        Utils.verifyMnemonic(mnemonic);
        try {
            Utils.verifyMnemonic('invalid mnemonic '.repeat(12));
            throw 'should error';
        } catch (e) {
            expect(e.message).toContain('NoSuchWord');
        }
    });

    it('converts address to hex and bech32', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = Utils.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        const bech32Address = Utils.hexToBech32(hexAddress, 'rms');

        expect(bech32Address).toBe(address);
    });

    it('converts hex public key to bech32 address', async () => {
        const hexPublicKey =
            '0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a';

        const address = Utils.hexPublicKeyToBech32Address(hexPublicKey, 'rms');

        expect(address).toBe(
            'rms1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx4aaacx',
        );
    });

    it('validates address', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';

        const isAddressValid = Utils.isAddressValid(address);

        expect(isAddressValid).toBeTruthy();
    });

    it('hash output id', async () => {
        const outputId =
            '0x0000000000000000000000000000000000000000000000000000000000000000000000000000';

        const accountId = Utils.computeAccountId(outputId);

        expect(accountId).toBe(
            '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345',
        );
    });

    it('account id to address', async () => {
        const accountId =
            '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345';

        const accountAddress = Utils.accountIdToBech32(accountId, 'rms');

        expect(accountAddress).toBe(
            'rms1pq8tc2r85fq8rxnsl2kdlsuyp6zhlfzskd7e22t6cnckdshhpse52a27mlc',
        );
    });

    it('compute foundry id', async () => {
        const accountAddress =
            '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345';
        const serialNumber = 0;
        const tokenSchemeType = 0;

        const foundryId = Utils.computeFoundryId(
            accountAddress,
            serialNumber,
            tokenSchemeType,
        );

        expect(foundryId).toBe(
            '0x080ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c33450000000000',
        );
    });
});
