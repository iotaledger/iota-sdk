import { Client, SecretManager, Utils } from '../../lib';
import '../customMatchers';
import 'dotenv/config';

const offlineClient = new Client({});

describe('Client utility methods', () => {
    // Requires "stronghold" in cargo toml iota-client features
    it.skip('generates and stores mnemonic', async () => {
        const mnemonic = Utils.generateMnemonic();

        const strongholdSecretManager = {
            stronghold: {
                password: 'some_hopefully_secure_password',
                snapshotPath: './stronghold',
            },
        };
        await expect(
            new SecretManager(strongholdSecretManager).storeMnemonic(mnemonic),
        ).resolves.toBe(null);
    });

    it('invalid mnemonic error', () => {
        try {
            Utils.verifyMnemonic('invalid mnemonic '.repeat(12));
            throw 'should error';
        } catch (e: any) {
            expect(e.payload.error).toContain('NoSuchWord');
        }
    });

    it('converts address to hex and bech32', async () => {
        const address =
            'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy';
        const hexAddress = Utils.bech32ToHex(address);

        expect(hexAddress.slice(0, 2)).toBe('0x');

        const bech32Address = await offlineClient.hexToBech32(
            hexAddress,
            'rms',
        );

        expect(bech32Address).toBe(address);
    });

    it('converts hex public key to bech32 address', async () => {
        const hexPublicKey =
            '0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a';

        const address = Utils.hexPublicKeyToBech32Address(
            hexPublicKey,
            'rms',
        );

        expect(address).toBeValidAddress();
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

        const aliasAddress = await offlineClient.aliasIdToBech32(
            aliasId,
            'rms',
        );

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

    it('sign and verify Ed25519', async () => {
        const secretManager = {
            mnemonic: Utils.generateMnemonic(),
        };

        // `IOTA` hex encoded
        const message = '0x494f5441';
        const signature = await new SecretManager(secretManager).signEd25519(
            message,
            // [44, 4218, 0, 0, 0] IOTA coin type, first account, first public address
            [
                { hardened: true, bs: [128, 0, 0, 44] },
                { hardened: true, bs: [128, 0, 16, 123] },
                { hardened: true, bs: [128, 0, 0, 0] },
                { hardened: true, bs: [128, 0, 0, 0] },
                { hardened: true, bs: [128, 0, 0, 0] },
            ],
        );

        const bech32Address = Utils.hexPublicKeyToBech32Address(
            signature.publicKey,
            'rms',
        );

        const pubKeyHash = Utils.bech32ToHex(bech32Address);

        const validSignature = Utils.verifyEd25519Signature(
            signature,
            message,
            { type: 0, pubKeyHash },
        );
        expect(validSignature).toBeTruthy();
    });
});
