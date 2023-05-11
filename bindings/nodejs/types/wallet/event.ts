import type { IOutputResponse, ITransactionPayload } from '@iota/types';
import type { OutputData } from './output';
import type { InclusionState } from './transaction';

// TODO where?
export type TransactionId = string;

/**
 * All of the wallet event types.
 */
enum WalletEventType {
    ConsolidationRequired = 0,
    LedgerAddressGeneration = 1,
    NewOutput = 2,
    SpentOutput = 3,
    TransactionInclusion = 4,
    TransactionProgress = 5
}

abstract class WalletEvent {
    private type: WalletEventType;

    constructor(type: WalletEventType) {
        this.type = type;
    }

    /**
     * The type of the wallet event.
     */
    getType(): WalletEventType {
        return this.type;
    }
}

// TransactionProgress = 5

class ConsolidationRequiredWalletEvent extends WalletEvent {
    constructor() {
        super(WalletEventType.ConsolidationRequired);
    }
}

class LedgerAddressGenerationWalletEvent extends WalletEvent {
    private address: string;

    constructor(address: string) {
        super(WalletEventType.LedgerAddressGeneration);
        this.address = address;
    }

    /**
    * The address.
    */
    getAddress(): string {
        return this.address;
    }
}

class NewOutputWalletEvent extends WalletEvent {
    output: OutputData;
    transaction?: ITransactionPayload;
    transactionInputs?: [IOutputResponse];

    constructor(output: OutputData,
        transaction?: ITransactionPayload,
        transactionInputs?: [IOutputResponse]) {
        super(WalletEventType.NewOutput);
        this.output = output;
        this.transaction = transaction;
        this.transactionInputs = transactionInputs;
    }

    /**
    * The output.
    */
    getOutput(): OutputData {
        return this.output;
    }

    /**
    * The transaction.
    */
    getTransaction(): ITransactionPayload | undefined {
        return this.transaction;
    }

    /**
    * The transaction inputs.
    */
    getTransactionInputs(): [IOutputResponse] | undefined {
        return this.transactionInputs;
    }
}

class SpentOutputWalletEvent extends WalletEvent {
    output: OutputData;

    constructor(output: OutputData,) {
        super(WalletEventType.SpentOutput);
        this.output = output;
    }

    /**
    * The output.
    */
    getOutput(): OutputData {
        return this.output;
    }
}

class TransactionInclusionWalletEvent extends WalletEvent {
    transactionId: TransactionId;
    inclusionState: InclusionState;

    constructor(transactionId: TransactionId,
        inclusionState: InclusionState) {
        super(WalletEventType.TransactionInclusion);
        this.transactionId = transactionId;
        this.inclusionState = inclusionState;
    }

    /**
    * The transaction ID.
    */
    getTransactionId(): TransactionId {
        return this.transactionId;
    }

    /**
    * The transaction ID.
    */
    getInclusionState(): InclusionState {
        return this.inclusionState;
    }
}

class TransactionProgressWalletEvent extends WalletEvent {
    constructor() {
        super(WalletEventType.TransactionProgress);
    }
}

export {
    WalletEvent,
    ConsolidationRequiredWalletEvent,
    LedgerAddressGenerationWalletEvent,
    NewOutputWalletEvent,
    SpentOutputWalletEvent,
    TransactionInclusionWalletEvent,
    TransactionProgressWalletEvent,
};