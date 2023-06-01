import { IClientOptions, CoinType } from '../client';
import { SecretManagerType } from '../secretManager/secretManager';

/** Options for the Wallet builder */
export interface WalletOptions {
    storagePath?: string;
    clientOptions?: IClientOptions;
    coinType?: CoinType;
    secretManager?: SecretManagerType;
}
