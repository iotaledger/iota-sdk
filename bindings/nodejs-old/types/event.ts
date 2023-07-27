import type {
    IOutputResponse,
    ITransactionPayload,
    ITransactionEssence,
} from '@iota/types';
import type { OutputData } from './output';
import type { InclusionState } from './transaction';
import type {
    InputSigningData,
    RemainderData,
} from './preparedTransactionData';

export type TransactionId = string;

class Event {
    /**
     * The account index for which the event was emitted.
     */
    accountIndex: number;
    /**
     * The wallet event.
     */
    event: WalletEvent;

    constructor(accountIndex: number, event: WalletEvent) {
        this.accountIndex = accountIndex;
        this.event = event;
    }
}

/**
 * All of the wallet event types.
 */
enum WalletEventType {
    ConsolidationRequired = 0,
    LedgerAddressGeneration = 1,
    NewOutput = 2,
    SpentOutput = 3,
    TransactionInclusion = 4,
    TransactionProgress = 5,
}

abstract class WalletEvent {
    /**
     * The type of the wallet event.
     */
    type: WalletEventType;

    constructor(type: WalletEventType) {
        this.type = type;
    }
}

class ConsolidationRequiredWalletEvent extends WalletEvent {
    constructor() {
        super(WalletEventType.ConsolidationRequired);
    }
}

class LedgerAddressGenerationWalletEvent extends WalletEvent {
    address: string;

    constructor(address: string) {
        super(WalletEventType.LedgerAddressGeneration);
        this.address = address;
    }
}

class NewOutputWalletEvent extends WalletEvent {
    output: OutputData;
    transaction?: ITransactionPayload;
    transactionInputs?: IOutputResponse[];

    constructor(
        output: OutputData,
        transaction?: ITransactionPayload,
        transactionInputs?: IOutputResponse[],
    ) {
        super(WalletEventType.NewOutput);
        this.output = output;
        this.transaction = transaction;
        this.transactionInputs = transactionInputs;
    }
}

class SpentOutputWalletEvent extends WalletEvent {
    output: OutputData;

    constructor(output: OutputData) {
        super(WalletEventType.SpentOutput);
        this.output = output;
    }
}

class TransactionInclusionWalletEvent extends WalletEvent {
    transactionId: TransactionId;
    inclusionState: InclusionState;

    constructor(transactionId: TransactionId, inclusionState: InclusionState) {
        super(WalletEventType.TransactionInclusion);
        this.transactionId = transactionId;
        this.inclusionState = inclusionState;
    }
}

/**
 * All of the transaction progress types.
 */
enum TransactionProgressType {
    SelectingInputs = 0,
    GeneratingRemainderDepositAddress = 1,
    PreparedTransaction = 2,
    PreparedTransactionEssenceHash = 3,
    SigningTransaction = 4,
    PerformingPow = 5,
    Broadcasting = 6,
}

class TransactionProgressWalletEvent extends WalletEvent {
    progress: TransactionProgress;

    constructor(progress: TransactionProgress) {
        super(WalletEventType.TransactionProgress);
        this.progress = progress;
    }
}

abstract class TransactionProgress {
    type: TransactionProgressType;

    constructor(type: TransactionProgressType) {
        this.type = type;
    }
}

class SelectingInputsProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.SelectingInputs);
    }
}

class GeneratingRemainderDepositAddressProgress extends TransactionProgress {
    address: string;

    constructor(address: string) {
        super(TransactionProgressType.GeneratingRemainderDepositAddress);
        this.address = address;
    }
}

class PreparedTransactionProgress extends TransactionProgress {
    essence: ITransactionEssence;
    inputsData: InputSigningData[];
    remainder?: RemainderData;

    constructor(
        essence: ITransactionEssence,
        inputsData: InputSigningData[],
        remainder?: RemainderData,
    ) {
        super(TransactionProgressType.PreparedTransaction);
        this.essence = essence;
        this.inputsData = inputsData;
        this.remainder = remainder;
    }
}

class PreparedTransactionEssenceHashProgress extends TransactionProgress {
    hash: string;

    constructor(hash: string) {
        super(TransactionProgressType.PreparedTransactionEssenceHash);
        this.hash = hash;
    }
}

class SigningTransactionProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.SigningTransaction);
    }
}

class PerformingPowProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.PerformingPow);
    }
}

class BroadcastingProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.Broadcasting);
    }
}

export {
    Event,
    WalletEventType,
    WalletEvent,
    ConsolidationRequiredWalletEvent,
    LedgerAddressGenerationWalletEvent,
    NewOutputWalletEvent,
    SpentOutputWalletEvent,
    TransactionInclusionWalletEvent,
    TransactionProgressWalletEvent,
    TransactionProgress,
    SelectingInputsProgress,
    GeneratingRemainderDepositAddressProgress,
    PreparedTransactionProgress,
    PreparedTransactionEssenceHashProgress,
    SigningTransactionProgress,
    PerformingPowProgress,
    BroadcastingProgress,
    TransactionProgressType,
};
