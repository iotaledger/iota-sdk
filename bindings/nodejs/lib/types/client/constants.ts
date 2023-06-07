export const IOTA_BECH32_HRP = 'iota';
export const IOTA_TESTNET_BECH32_HRP = 'atoi';
export const SHIMMER_BECH32_HRP = 'smr';
export const SHIMMER_TESTNET_BECH32_HRP = 'rms';

/** BIP44 Coin Types for IOTA and Shimmer. */
export enum CoinType {
    IOTA = 4218,
    Shimmer = 4219,
    Ether = 60,
}

export const HD_WALLET_TYPE = 44;
export const HARDEN_MASK = (1 << 31) >>> 0;
