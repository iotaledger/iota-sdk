import type { HexEncodedString } from '@iota/types';
import type { IGenerateAddressesOptions } from '../../client/generateAddressesOptions';
import type {
    IPreparedTransactionData,
    IBip32Chain,
} from '../../client/preparedTransactionData';

export interface __GenerateEd25519AddressesMethod__ {
    name: 'generateEd25519Addresses';
    data: {
        options: IGenerateAddressesOptions;
    };
}

export interface __GenerateEvmAddressesMethod__ {
    name: 'generateEvmAddresses';
    data: {
        options: IGenerateAddressesOptions;
    };
}

export interface __SignTransactionMethod__ {
    name: 'signTransaction';
    data: {
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __SignatureUnlockMethod__ {
    name: 'signatureUnlock';
    data: {
        transactionEssenceHash: HexEncodedString;
        chain: IBip32Chain;
    };
}

export interface __StoreMnemonicMethod__ {
    name: 'storeMnemonic';
    data: {
        mnemonic: string;
    };
}

export interface __SignEd25519Method__ {
    name: 'signEd25519';
    data: {
        message: HexEncodedString;
        chain: IBip32Chain;
    };
}

export interface __SignEvmMethod__ {
    name: 'signEvm';
    data: {
        message: HexEncodedString;
        chain: IBip32Chain;
    };
}

export interface __GetLedgerNanoStatusMethod__ {
    name: 'getLedgerNanoStatus';
}
