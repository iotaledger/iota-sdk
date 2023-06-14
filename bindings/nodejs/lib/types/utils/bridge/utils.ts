import {
    Ed25519Address,
    Ed25519Signature,
    HexEncodedString,
    Block,
    TransactionEssence,
} from '../../';

export interface __GenerateMnemonicMethod__ {
    name: 'generateMnemonic';
}

export interface __MnemonicToHexSeedMethod__ {
    name: 'mnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __ComputeAliasIdMethod__ {
    name: 'computeAliasId';
    data: {
        outputId: string;
    };
}

export interface __ComputeNftIdMethod__ {
    name: 'computeNftId';
    data: {
        outputId: string;
    };
}

export interface __ComputeFoundryIdMethod__ {
    name: 'computeFoundryId';
    data: {
        aliasAddress: string;
        serialNumber: number;
        tokenSchemeKind: number;
    };
}

export interface __ParseBech32AddressMethod__ {
    name: 'parseBech32Address';
    data: {
        address: string;
    };
}

export interface __BlockIdMethod__ {
    name: 'blockId';
    data: {
        block: Block;
    };
}

export interface __Bech32ToHexMethod__ {
    name: 'bech32ToHex';
    data: {
        bech32: string;
    };
}

export interface __HexToBech32Method__ {
    name: 'hexToBech32';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __AliasIdToBech32Method__ {
    name: 'aliasIdToBech32';
    data: {
        aliasId: string;
        bech32Hrp?: string;
    };
}

export interface __NftIdToBech32Method__ {
    name: 'nftIdToBech32';
    data: {
        nftId: string;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressMethod__ {
    name: 'hexPublicKeyToBech32Address';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __IsAddressValidMethod__ {
    name: 'isAddressValid';
    data: {
        address: string;
    };
}

export interface __HashTransactionEssenceMethod__ {
    name: 'hashTransactionEssence';
    data: {
        essence: TransactionEssence;
    };
}

export interface __VerifyEd25519Method__ {
    name: 'verifyEd25519';
    data: {
        signature: Ed25519Signature;
        message: HexEncodedString;
    };
}

export interface __VerifyEvmMethod__ {
    name: 'verifyEvm';
    data: {
        public_key: HexEncodedString;
        signature: HexEncodedString;
        message: HexEncodedString;
    };
}

export type __VerifyMnemonicMethod__ = {
    name: 'verifyMnemonic';
    data: { mnemonic: string };
};

export type __FaucetMethod__ = {
    name: 'faucet';
    data: {
        url: string;
        address: string;
    };
};
