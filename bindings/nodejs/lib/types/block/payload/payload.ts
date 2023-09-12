// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the block payload types.
 */
enum PayloadType {
    /** A tagged data payload. */
    TaggedData = 5,
    /** A transaction payload. */
    Transaction = 6,
}

/**
 * The base class for block payloads.
 */
abstract class Payload {
    /**
     * Get the type of payload.
     */
    readonly type: PayloadType;

    /**
     * @param type The type of payload.
     */
    constructor(type: PayloadType) {
        this.type = type;
    }
}

export { PayloadType, Payload };
