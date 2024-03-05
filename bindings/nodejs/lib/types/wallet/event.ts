// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { OutputData } from './output';
import { InclusionState } from './transaction';
import { InputSigningData, Remainder } from '../client';
import { Transaction, SignedTransactionPayload, TransactionId } from '../block';
import { OutputResponse } from '../models';
import { HexEncodedString } from '../utils';

/**
 * All of the wallet event types.
 */
enum WalletEventType {
    /** Nano Ledger has generated an address. */
    LedgerAddressGeneration = 0,
    /** A new output was created. */
    NewOutput = 1,
    /** An output was spent. */
    SpentOutput = 2,
    /** A transaction was included into the ledger. */
    TransactionInclusion = 3,
    /** A progress update while submitting a transaction. */
    TransactionProgress = 4,
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
    /** Signing the transaction. */
    SigningTransaction = 3,
    /** Prepared transaction signing hash hex encoded, required for blindsigning with a Ledger Nano. */
    PreparedTransactionSigningHash = 4,
    /** Prepared block signing input, required for blindsigning with a Ledger Nano. */
    PreparedBlockSigningInput = 5,
    /** Broadcasting. */
    Broadcasting = 6,
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
    remainders?: Remainder[];

    /**
     * @param transaction The prepared transaction.
     * @param inputsData Input signing parameters.
     * @param remainders Remainder outputs parameters.
     */
    constructor(
        transaction: Transaction,
        inputsData: InputSigningData[],
        remainders?: Remainder[],
    ) {
        super(TransactionProgressType.PreparedTransaction);
        this.transaction = transaction;
        this.inputsData = inputsData;
        this.remainders = remainders;
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
 * A 'prepared transaction hash' progress.
 */
class PreparedTransactionSigningHashProgress extends TransactionProgress {
    signingHash: HexEncodedString;

    /**
     * @param signingHash The signing hash of the transaction.
     */
    constructor(signingHash: HexEncodedString) {
        super(TransactionProgressType.PreparedTransactionSigningHash);
        this.signingHash = signingHash;
    }
}

/**
 * A 'prepared block input' progress.
 */
class PreparedBlockSigningInputProgress extends TransactionProgress {
    blockSigningInput: HexEncodedString;

    /**
     * @param signingHash The signing hash of the block.
     */
    constructor(signingInput: HexEncodedString) {
        super(TransactionProgressType.PreparedBlockSigningInput);
        this.blockSigningInput = signingInput;
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
    WalletEventType,
    WalletEvent,
    LedgerAddressGenerationWalletEvent,
    NewOutputWalletEvent,
    SpentOutputWalletEvent,
    TransactionInclusionWalletEvent,
    TransactionProgressWalletEvent,
    TransactionProgress,
    SelectingInputsProgress,
    GeneratingRemainderDepositAddressProgress,
    PreparedTransactionProgress,
    SigningTransactionProgress,
    PreparedTransactionSigningHashProgress,
    PreparedBlockSigningInputProgress,
    BroadcastingProgress,
    TransactionProgressType,
};
