import type { ISimpleTokenScheme } from '@iota/types';
import { Feature } from '../../../lib';
import type { BasicOutputBuilderOptions } from './basicOutputOptions';

/**
 * Options for building a Foundry Output
 */
export interface FoundryOutputBuilderOptions extends BasicOutputBuilderOptions {
    /**
     * The serial number of the foundry with respect to the controlling alias.
     */
    serialNumber: number;
    tokenScheme: ISimpleTokenScheme;
    immutableFeatures?: Feature[];
}
