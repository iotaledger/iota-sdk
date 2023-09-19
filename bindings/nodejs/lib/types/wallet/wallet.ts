import type { IClientOptions, CoinType } from '../client';
import type { SecretManagerType } from '../secret_manager';

/** Options for the Wallet builder. */
export interface WalletOptions {
    /** The path to the wallet database. */
    storagePath?: string;
    /** The node client options. */
    clientOptions?: IClientOptions;
    /** The type of coin stored with the wallet. */
    coinType?: CoinType;
    /** The secret manager to use. */
    secretManager?: SecretManagerType;
}
