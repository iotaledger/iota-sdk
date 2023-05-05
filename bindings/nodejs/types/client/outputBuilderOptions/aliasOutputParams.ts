import { HexEncodedString } from '@iota/types';
import { Feature } from '../../../lib';
import type { BasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building an Alias Output
 */
export interface AliasOutputBuilderParams extends BasicOutputBuilderParams {
    aliasId: HexEncodedString;
    stateIndex?: number;
    stateMetadata?: HexEncodedString;
    foundryCounter?: number;
    immutableFeatures?: Feature[];
}
