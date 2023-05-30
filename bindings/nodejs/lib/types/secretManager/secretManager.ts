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
