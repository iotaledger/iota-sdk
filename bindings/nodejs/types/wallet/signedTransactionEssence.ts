import type { ITransactionPayload } from '@iota/types';
import { IInputSigningData } from '../client';

/** The signed transaction with inputs data */
export interface SignedTransactionEssence {
    transactionPayload: ITransactionPayload;
    inputsData: IInputSigningData;
}
