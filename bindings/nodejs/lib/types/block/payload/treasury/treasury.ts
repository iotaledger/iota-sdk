import { Type } from 'class-transformer';
import { TreasuryInput } from '../../input';
import { TreasuryOutput } from '../../output';
import { Payload, PayloadType } from '../payload';

/**
 * A treasury transaction payload.
 */
class TreasuryTransactionPayload extends Payload {
    /**
     * The input of this transaction.
     */
    @Type(() => TreasuryInput)
    input: TreasuryInput;
    /**
     * The output of this transaction.
     */
    @Type(() => TreasuryOutput)
    output: TreasuryOutput;

    /**
     * @param input A Treasury input.
     * @param output A Treasury output.
     */
    constructor(input: TreasuryInput, output: TreasuryOutput) {
        super(PayloadType.Transaction);
        this.input = input;
        this.output = output;
    }
}

export { TreasuryTransactionPayload };
