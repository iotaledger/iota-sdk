// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { OutputData } from './output';
import { InclusionState } from './transaction';
import { InputSigningData, Remainder } from '../client';
import { TransactionEssence, TransactionPayload } from '../block';
import { OutputResponse } from '../models';

/**
 * TODO.
 */
export type TransactionId = string;

/**
 * TODO.
 */
class Event {
    /**
     * The account index for which the event was emitted.
     */
    accountIndex: number;
    /**
     * TODO.
     */
    event: WalletEvent;

    /**
     * TODO.
     */
    constructor(accountIndex: number, event: WalletEvent) {
        this.accountIndex = accountIndex;
        this.event = event;
    }
}

/**
 * All of the wallet event types.
 */
enum WalletEventType {
    /** TODO */
    ConsolidationRequired = 0,
    /** TODO */
    LedgerAddressGeneration = 1,
    /** TODO */
    NewOutput = 2,
    /** TODO */
    SpentOutput = 3,
    /** TODO */
    TransactionInclusion = 4,
    /** TODO */
    TransactionProgress = 5,
}

/**
 * TODO.
 */
abstract class WalletEvent {
    type: WalletEventType;

    /**
     * TODO.
     */
    constructor(type: WalletEventType) {
        this.type = type;
    }
}

/**
 * TODO.
 */
class ConsolidationRequiredWalletEvent extends WalletEvent {
    /**
     * TODO.
     */
    constructor() {
        super(WalletEventType.ConsolidationRequired);
    }
}

/**
 * TODO.
 */
class LedgerAddressGenerationWalletEvent extends WalletEvent {
    address: string;

    /**
     * TODO.
     */
    constructor(address: string) {
        super(WalletEventType.LedgerAddressGeneration);
        this.address = address;
    }
}

/**
 * TODO.
 */
class NewOutputWalletEvent extends WalletEvent {
    /** TODO */
    output: OutputData;
    /** TODO */
    transaction?: TransactionPayload;
    /** TODO */
    transactionInputs?: OutputResponse[];

    /**
     * TODO.
     */
    constructor(
        output: OutputData,
        transaction?: TransactionPayload,
        transactionInputs?: OutputResponse[],
    ) {
        super(WalletEventType.NewOutput);
        this.output = output;
        this.transaction = transaction;
        this.transactionInputs = transactionInputs;
    }
}

/**
 * TODO.
 */
class SpentOutputWalletEvent extends WalletEvent {
    /** TODO */
    output: OutputData;

    /**
     * TODO.
     */
    constructor(output: OutputData) {
        super(WalletEventType.SpentOutput);
        this.output = output;
    }
}

/**
 * TODO.
 */
class TransactionInclusionWalletEvent extends WalletEvent {
    /** TODO */
    transactionId: TransactionId;
    /** TODO */
    inclusionState: InclusionState;

    /**
     * TODO.
     */
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
    /** TODO */
    SelectingInputs = 0,
    /** TODO */
    GeneratingRemainderDepositAddress = 1,
    /** TODO */
    PreparedTransaction = 2,
    /** TODO */
    PreparedTransactionEssenceHash = 3,
    /** TODO */
    SigningTransaction = 4,
    /** TODO */
    PerformingPow = 5,
    /** TODO */
    Broadcasting = 6,
}

/**
 * TODO.
 */
class TransactionProgressWalletEvent extends WalletEvent {
    /** TODO */
    progress: TransactionProgress;

    constructor(progress: TransactionProgress) {
        super(WalletEventType.TransactionProgress);
        this.progress = progress;
    }
}

/**
 * TODO.
 */
abstract class TransactionProgress {
    /** TODO */
    type: TransactionProgressType;

    /**
     * TODO.
     */
    constructor(type: TransactionProgressType) {
        this.type = type;
    }
}

/**
 * TODO.
 */
class SelectingInputsProgress extends TransactionProgress {
    /**
     * TODO.
     */
    constructor() {
        super(TransactionProgressType.SelectingInputs);
    }
}

/**
 * TODO.
 */
class GeneratingRemainderDepositAddressProgress extends TransactionProgress {
    /** TODO */
    address: string;

    /**
     * TODO.
     */
    constructor(address: string) {
        super(TransactionProgressType.GeneratingRemainderDepositAddress);
        this.address = address;
    }
}

/**
 * TODO.
 */
class PreparedTransactionProgress extends TransactionProgress {
    /** TODO */
    essence: TransactionEssence;
    /** TODO */
    inputsData: InputSigningData[];
    /** TODO */
    remainder?: Remainder;

    /**
     * TODO.
     */
    constructor(
        essence: TransactionEssence,
        inputsData: InputSigningData[],
        remainder?: Remainder,
    ) {
        super(TransactionProgressType.PreparedTransaction);
        this.essence = essence;
        this.inputsData = inputsData;
        this.remainder = remainder;
    }
}

/**
 * TODO.
 */
class PreparedTransactionEssenceHashProgress extends TransactionProgress {
    /** TODO */
    hash: string;

    /**
     * TODO.
     */
    constructor(hash: string) {
        super(TransactionProgressType.PreparedTransactionEssenceHash);
        this.hash = hash;
    }
}

/**
 * TODO.
 */
class SigningTransactionProgress extends TransactionProgress {
    /**
     * TODO.
     */
    constructor() {
        super(TransactionProgressType.SigningTransaction);
    }
}

/**
 * TODO.
 */
class PerformingPowProgress extends TransactionProgress {
    /**
     * TODO.
     */
    constructor() {
        super(TransactionProgressType.PerformingPow);
    }
}

/**
 * TODO.
 */
class BroadcastingProgress extends TransactionProgress {
    /**
     * TODO.
     */
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
