import type { HexEncodedString } from '@iota/types';
import { Feature } from '../../../lib';
import type { BasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building an Nft Output
 */
export interface NftOutputBuilderParams extends BasicOutputBuilderParams {
    nftId: HexEncodedString;
    immutableFeatures?: Feature[];
}
