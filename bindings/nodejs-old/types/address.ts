import type { HexEncodedAmount } from '@iota/types';
import type { CoinType } from './account';

/** Address Types */
export enum AddressType {
    Ed25519 = 'Ed25519',
    Alias = 'Alias',
    Nft = 'Nft',
}

/** An Address of the Account */
export interface Address {
    address: string;
    keyIndex: number;
    internal: boolean;
    used: boolean;
}

/** Address with a base coin amount */
export interface SendParams {
    address: string;
    amount: string;
    returnAddress?: string;
    expiration?: number;
}

/** Address with unspent outputs */
export interface AddressWithUnspentOutputs {
    address: string;
    keyIndex: number;
    internal: boolean;
    outputIds: string[];
}

/** Address with native tokens */
export interface SendNativeTokensParams {
    address: string;
    nativeTokens: [string, HexEncodedAmount][];
    returnAddress?: string;
    expiration?: number;
}

/** Address with an NftId */
export interface SendNftParams {
    address: string;
    nftId: string;
}

/** Options for address generation, useful with a Ledger Nano SecretManager */
export interface GenerateAddressOptions {
    /**
     * Internal addresses
     */
    internal?: boolean;
    ledgerNanoPrompt?: boolean;
}

export interface GenerateAddressesOptions {
    coinType?: CoinType;
    accountIndex?: number;
    range?: IRange;
    /**
     * Bech32 human readable part
     */
    bech32Hrp?: string;
    options?: GenerateAddressOptions;
}

export interface IRange {
    start: number;
    end: number;
}
