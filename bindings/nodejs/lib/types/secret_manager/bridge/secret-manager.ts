// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { UnsignedBlock } from '../../block';
import { CoinType } from '../../client';
import type { GenerateAddressesOptions } from '../../client/generate-addresses-options';
import type { PreparedTransactionData } from '../../client/prepared-transaction-data';
import { HexEncodedString } from '../../utils';
import { Bip44 } from '../secret-manager';

export interface __GenerateEd25519AddressMethod__ {
    name: 'generateEd25519Address';
    data: {
        coinType: CoinType;
        bech32Hrp: string;
        accountIndex?: number;
        addressIndex?: number;
        internal?: boolean;
        ledgerNanoPrompt?: boolean;
    };
}

export interface __GenerateEd25519AddressesMethod__ {
    name: 'generateEd25519Addresses';
    data: {
        options: GenerateAddressesOptions;
    };
}

export interface __GenerateEvmAddressesMethod__ {
    name: 'generateEvmAddresses';
    data: {
        options: GenerateAddressesOptions;
    };
}

export interface __SignTransactionMethod__ {
    name: 'signTransaction';
    data: {
        preparedTransactionData: PreparedTransactionData;
    };
}

export interface __SignBlockMethod__ {
    name: 'signBlock';
    data: {
        unsignedBlock: UnsignedBlock;
        chain: Bip44;
    };
}

export interface __SignatureUnlockMethod__ {
    name: 'signatureUnlock';
    data: {
        transactionSigningHash: HexEncodedString;
        chain: Bip44;
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
        chain: Bip44;
    };
}

export interface __SignSecp256k1EcdsaMethod__ {
    name: 'signSecp256k1Ecdsa';
    data: {
        message: HexEncodedString;
        chain: Bip44;
    };
}

export interface __GetLedgerNanoStatusMethod__ {
    name: 'getLedgerNanoStatus';
}

export interface __SetStrongholdPasswordMethod__ {
    name: 'setStrongholdPassword';
    data: { password: string };
}

export interface __ChangeStrongholdPasswordMethod__ {
    name: 'changeStrongholdPassword';
    data: { password: string };
}

export interface __ClearStrongholdPasswordMethod__ {
    name: 'clearStrongholdPassword';
}
