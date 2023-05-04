import type { FeatureTypes, HexEncodedString } from '@iota/types';
import type { IBasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building an Alias Output
 */
export interface IAliasOutputBuilderParams extends IBasicOutputBuilderParams {
    aliasId: HexEncodedString;
    stateIndex?: number;
    stateMetadata?: HexEncodedString;
    foundryCounter?: number;
    immutableFeatures?: FeatureTypes[];
}
