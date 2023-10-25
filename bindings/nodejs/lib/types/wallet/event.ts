// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { OutputData } from './output';
import { InclusionState } from './transaction';
import { InputSigningData, Remainder } from '../client';
import { Transaction, SignedTransactionPayload } from '../block';
import { OutputResponse } from '../models';
import { HexEncodedString } from '../utils';

/**
 * A Transaction ID represented as hex-encoded string.
 */
export type TransactionId = string;

/**
 * An wallet account event.
 */
class Event {
    /**
     * The account index for which the event was emitted.
     */
    accountIndex: number;
    /**
     * The wallet event.
     */
    event: WalletEvent;

    /**
     * @param accountIndex The account index.
     * @param event The wallet event.
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
    /** Consolidation is required. */
    ConsolidationRequired = 0,
    /** Nano Ledger has generated an address. */
    LedgerAddressGeneration = 1,
    /** A new output was created. */
    NewOutput = 2,
    /** An output was spent. */
    SpentOutput = 3,
    /** A transaction was included into the ledger. */
    TransactionInclusion = 4,
    /** A progress update while submitting a transaction. */
    TransactionProgress = 5,
}

/**
 * The base class for wallet events.
 */
abstract class WalletEvent {
    type: WalletEventType;

    /**
     * @param type The type of wallet event.
     */
    constructor(type: WalletEventType) {
        this.type = type;
    }
}

/**
 * A 'consolidation required' wallet event.
 */
class ConsolidationRequiredWalletEvent extends WalletEvent {
    constructor() {
        super(WalletEventType.ConsolidationRequired);
    }
}

/**
 * A 'ledger address generation' wallet event.
 */
class LedgerAddressGenerationWalletEvent extends WalletEvent {
    address: string;

    /**
     * @param address The generated address.
     */
    constructor(address: string) {
        super(WalletEventType.LedgerAddressGeneration);
        this.address = address;
    }
}

/**
 * A 'new output' wallet event.
 */
class NewOutputWalletEvent extends WalletEvent {
    output: OutputData;
    transaction?: SignedTransactionPayload;
    transactionInputs?: OutputResponse[];

    /**
     * @param output The new output.
     * @param transaction The transaction that created the output. Might be pruned and not available.
     * @param transactionInputs The inputs for the transaction that created the output. Might be pruned and not available.
     */
    constructor(
        output: OutputData,
        transaction?: SignedTransactionPayload,
        transactionInputs?: OutputResponse[],
    ) {
        super(WalletEventType.NewOutput);
        this.output = output;
        this.transaction = transaction;
        this.transactionInputs = transactionInputs;
    }
}

/**
 * A 'spent output' wallet event.
 */
class SpentOutputWalletEvent extends WalletEvent {
    output: OutputData;

    /**
     * @param output The spent output.
     */
    constructor(output: OutputData) {
        super(WalletEventType.SpentOutput);
        this.output = output;
    }
}

/**
 * A 'transaction inclusion' wallet event.
 */
class TransactionInclusionWalletEvent extends WalletEvent {
    transactionId: TransactionId;
    inclusionState: InclusionState;

    /**
     * @param transactionId The transaction ID.
     * @param inclusionState The inclusion state of the transaction.
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
    /** Performing input selection. */
    SelectingInputs = 0,
    /** Generating remainder value deposit address. */
    GeneratingRemainderDepositAddress = 1,
    /** Prepared transaction. */
    PreparedTransaction = 2,
    /** Prepared transaction signing hash hex encoded, required for blindsigning with a Ledger Nano. */
    PreparedTransactionSigningHash = 3,
    /** Signing the transaction. */
    SigningTransaction = 4,
    /** Broadcasting. */
    Broadcasting = 5,
}

/**
 * A 'transaction progress' wallet event.
 */
class TransactionProgressWalletEvent extends WalletEvent {
    progress: TransactionProgress;

    /**
     * @param progress The progress of the transaction.
     */
    constructor(progress: TransactionProgress) {
        super(WalletEventType.TransactionProgress);
        this.progress = progress;
    }
}

/**
 * The base class for transaction progresses.
 */
abstract class TransactionProgress {
    type: TransactionProgressType;

    /**
     * @param type The type of transaction progress.
     */
    constructor(type: TransactionProgressType) {
        this.type = type;
    }
}

/**
 * A 'selecting inputs' progress.
 */
class SelectingInputsProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.SelectingInputs);
    }
}

/**
 * A 'generating remainder deposit address' progress.
 */
class GeneratingRemainderDepositAddressProgress extends TransactionProgress {
    address: string;

    /**
     * @param address The generated remainder deposit address.
     */
    constructor(address: string) {
        super(TransactionProgressType.GeneratingRemainderDepositAddress);
        this.address = address;
    }
}

/**
 * A 'prepared transaction' progress.
 */
class PreparedTransactionProgress extends TransactionProgress {
    transaction: Transaction;
    inputsData: InputSigningData[];
    remainder?: Remainder;

    /**
     * @param transaction The prepared transaction.
     * @param inputsData Input signing parameters.
     * @param remainder Remainder output parameters.
     */
    constructor(
        transaction: Transaction,
        inputsData: InputSigningData[],
        remainder?: Remainder,
    ) {
        super(TransactionProgressType.PreparedTransaction);
        this.transaction = transaction;
        this.inputsData = inputsData;
        this.remainder = remainder;
    }
}

/**
 * A 'prepared transaction hash' progress.
 */
class PreparedTransactionSigningHashProgress extends TransactionProgress {
    signingHash: HexEncodedString;

    /**
     * @param signing_hash The signing hash of the transaction.
     */
    constructor(signingHash: HexEncodedString) {
        super(TransactionProgressType.PreparedTransactionSigningHash);
        this.signingHash = signingHash;
    }
}

/**
 * A 'signing transaction' progress.
 */
class SigningTransactionProgress extends TransactionProgress {
    constructor() {
        super(TransactionProgressType.SigningTransaction);
    }
}

/**
 * A 'broadcasting' progress.
 */
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
    PreparedTransactionSigningHashProgress,
    SigningTransactionProgress,
    BroadcastingProgress,
    TransactionProgressType,
};
