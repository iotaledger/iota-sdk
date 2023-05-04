import type { FeatureTypes, HexEncodedString } from '@iota/types';
import type { IBasicOutputBuilderOptions } from './basicOutputParams';

/**
 * Options for building an Nft Output
 */
export interface INftOutputBuilderOptions extends IBasicOutputBuilderOptions {
    nftId: HexEncodedString;
    immutableFeatures?: FeatureTypes[];
}
