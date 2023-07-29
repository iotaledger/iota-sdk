import { IClientOptions, CoinType } from '../client';
import { SecretManagerType } from '../secret_manager/secret-manager';

/** Options for the Wallet builder */
export interface WalletOptions {
    /** TODO */
    storagePath?: string;
    /** TODO */
    clientOptions?: IClientOptions;
    /** TODO */
    coinType?: CoinType;
    /** TODO */
    secretManager?: SecretManagerType;
}
