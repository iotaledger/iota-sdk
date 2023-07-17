// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../utils';

/** Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator. */
export interface LedgerNanoSecretManager {
    /** boolean indicates whether it's a simulator or not. */
    ledgerNano: boolean;
}

/** Secret manager that uses a mnemonic. */
export interface MnemonicSecretManager {
    mnemonic: string;
}

/** Secret manager that uses a seed. */
export interface SeedSecretManager {
    hexSeed: string;
}

/** Placeholder secret manager that can't do anything. */
export type PlaceholderSecretManager = 'placeholder';

/** Secret manager that uses Stronghold. */
export interface StrongholdSecretManager {
    stronghold: {
        password?: string;
        snapshotPath?: string;
    };
}

/** Supported secret managers */
export type SecretManagerType =
    | LedgerNanoSecretManager
    | MnemonicSecretManager
    | StrongholdSecretManager
    | PlaceholderSecretManager;

export interface Secp256k1EcdsaSignature {
    /**
     * The public key.
     */
    publicKey: HexEncodedString;
    /**
     * The signature.
     */
    signature: HexEncodedString;
}

export interface Bip44 {
    coinType?: number;
    account?: number;
    change?: number;
    addressIndex?: number;
}
