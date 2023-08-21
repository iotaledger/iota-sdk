// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { OutputData } from './output';
import { InclusionState } from './transaction';
import { InputSigningData, Remainder } from '../client';
import { TransactionEssence, TransactionPayload } from '../block';
import { OutputResponse } from '../models';

export type TransactionId = string;

class Event {
    /**
     * The account index for which the event was emitted.
     */
    accountIndex: number;
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
    transaction?: TransactionPayload;
    transactionInputs?: OutputResponse[];

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
    Broadcasting = 5,
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
    essence: TransactionEssence;
    inputsData: InputSigningData[];
    remainder?: Remainder;

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
    BroadcastingProgress,
    TransactionProgressType,
};
