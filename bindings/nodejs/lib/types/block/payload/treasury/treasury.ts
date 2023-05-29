import { TreasuryInput } from '../../input';
import { TreasuryOutput } from '../../output';
import { Payload, PayloadType } from '../payload';

/**
 * Receipt payload.
 */
class TreasuryTransactionPayload extends Payload {
    /**
     * The input of this transaction.
     */
    input: TreasuryInput;
    /**
     * The output of this transaction.
     */
    output: TreasuryOutput;

    constructor(input: TreasuryInput, output: TreasuryOutput) {
        super(PayloadType.Transaction);
        this.input = input;
        this.output = output;
    }
}

export { TreasuryTransactionPayload };
