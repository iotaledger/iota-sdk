import type { FeatureTypes, ISimpleTokenScheme } from '@iota/types';
import type { IBasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building a Foundry Output
 */
export interface IFoundryOutputBuilderParams
    extends IBasicOutputBuilderParams {
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    serialNumber: number;
    tokenScheme: ISimpleTokenScheme;
    immutableFeatures?: FeatureTypes[];
}
