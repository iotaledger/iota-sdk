import { INativeToken } from '@iota/types';
import { UnlockCondition, Feature } from '../../../lib';

/**
 * Options for building a Basic Output
 */
export interface BasicOutputBuilderParams {
    /**
     * If not provided, minimum storage deposit will be used
     */
    amount?: string;
    /**
     * The native tokens to be held by the output.
     */
    nativeTokens?: INativeToken[];
    /**
     * The unlock conditions for the output.
     */
    unlockConditions: UnlockCondition[];
    /**
     * Features to be contained by the output.
     */
    features?: Feature[];
}
