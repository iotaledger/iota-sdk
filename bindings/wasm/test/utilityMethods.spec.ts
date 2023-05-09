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
            expect(e.payload.error).toContain('NoSuchWord');
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
            '0x00000000000000000000000000000000000000000000000000000000000000000000';

        const aliasId = Utils.computeAliasId(outputId);

        expect(aliasId).toBe(
            '0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f',
        );
    });

    it('alias id to address', async () => {
        const aliasId =
            '0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f';

        const aliasAddress = Utils.aliasIdToBech32(aliasId, 'rms');

        expect(aliasAddress).toBe(
            'rms1pr8swlf8v6rt5exqgp9eavk324t8sggnckseshex9dc0n9jd8w7h7wcnhn7',
        );
    });

    it('compute foundry id', async () => {
        const aliasAddress =
            '0xcf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f';
        const serialNumber = 0;
        const tokenSchemeKind = 0;

        const foundryId = Utils.computeFoundryId(
            aliasAddress,
            serialNumber,
            tokenSchemeKind,
        );

        expect(foundryId).toBe(
            '0x08cf077d276686ba64c0404b9eb2d15556782113c5a1985f262b70f9964d3bbd7f0000000000',
        );
    });
});
