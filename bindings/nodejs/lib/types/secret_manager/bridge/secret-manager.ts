// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IGenerateAddressesOptions } from '../../client/generate-addresses-options';
import type {
    PreparedTransactionData,
    IBip32Chain,
} from '../../client/prepared-transaction-data';
import { HexEncodedString } from '../../utils';

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
        preparedTransactionData: PreparedTransactionData;
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

export interface __SignSecp256k1EcdsaMethod__ {
    name: 'signSecp256k1Ecdsa';
    data: {
        message: HexEncodedString;
        chain: IBip32Chain;
    };
}

export interface __GetLedgerNanoStatusMethod__ {
    name: 'getLedgerNanoStatus';
}
