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
    /** The underlying mnemonic. */
    mnemonic: string;
}

/** Secret manager that uses a seed. */
export interface SeedSecretManager {
    /** The underlying seed. */
    hexSeed: string;
}

/** Placeholder secret manager that can't do anything. */
export type PlaceholderSecretManager = 'placeholder';

/** Secret manager that uses Stronghold. */
export interface StrongholdSecretManager {
    /** The underlying Stronghold snapshot. */
    stronghold: {
        password?: string;
        snapshotPath?: string;
    };
}

/** Secret manager based on a single ED25519 private key. */
export interface PrivateKeySecretManager {
    privateKey: HexEncodedString;
}

/** Supported secret managers */
export type SecretManagerType =
    | LedgerNanoSecretManager
    | MnemonicSecretManager
    | SeedSecretManager
    | StrongholdSecretManager
    | PrivateKeySecretManager
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

/** A BIP44 chain. */
export interface Bip44 {
    /** The coin type segment. */
    coinType?: number;
    /** The account segment. */
    account?: number;
    /** The change segment. */
    change?: number;
    /** The address index segment. */
    addressIndex?: number;
}
