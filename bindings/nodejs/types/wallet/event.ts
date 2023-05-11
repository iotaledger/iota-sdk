import type { IOutputResponse, ITransactionPayload } from '@iota/types';
import type { OutputData } from './output';

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

// SpentOutput = 3,
// TransactionInclusion = 4,
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
    constructor() {
        super(WalletEventType.SpentOutput);
    }
}

class TransactionInclusionWalletEvent extends WalletEvent {
    constructor() {
        super(WalletEventType.TransactionInclusion);
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