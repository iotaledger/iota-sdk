import type { HexEncodedString } from '@iota/types';
import { Feature } from '../../../lib';
import type { BasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building an Nft Output
 */
export interface NftOutputBuilderOptions extends BasicOutputBuilderOptions {
    nftId: HexEncodedString;
    immutableFeatures?: Feature[];
}
