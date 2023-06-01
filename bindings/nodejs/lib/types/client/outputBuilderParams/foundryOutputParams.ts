import { Feature, SimpleTokenScheme } from '../../';
import type { BasicOutputBuilderParams } from './basicOutputParams';

/**
 * Options for building a Foundry Output
 */
export interface FoundryOutputBuilderParams extends BasicOutputBuilderParams {
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    serialNumber: number;
    tokenScheme: SimpleTokenScheme;
    immutableFeatures?: Feature[];
}
