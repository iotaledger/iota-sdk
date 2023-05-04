import type { FeatureTypes, HexEncodedString } from '@iota/types';
import type { IBasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building an Nft Output
 */
export interface INftOutputBuilderParams extends IBasicOutputBuilderParams {
    nftId: HexEncodedString;
    immutableFeatures?: FeatureTypes[];
}
