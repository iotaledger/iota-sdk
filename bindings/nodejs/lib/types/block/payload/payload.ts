// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the payload types.
 */
enum PayloadType {
    /** A milestone payload. */
    Milestone = 7,
    /** A tagged data payload. */
    TaggedData = 5,
    /** A transaction payload. */
    Transaction = 6,
    /** A treasury transaction payload. */
    TreasuryTransaction = 4,
}

/**
 * TODO.
 */
abstract class Payload {
    readonly type: PayloadType;

    /**
     * TODO.
     */
    constructor(type: PayloadType) {
        this.type = type;
    }

    /**
     * Get the type of payload.
     */
    getType(): PayloadType {
        return this.type;
    }
}

export { PayloadType, Payload };
