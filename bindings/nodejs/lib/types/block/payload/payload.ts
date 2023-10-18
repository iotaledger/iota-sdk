// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * All of the block payload types.
 */
enum PayloadType {
    /** A tagged data payload. */
    TaggedData = 0,
    /** A transaction payload. */
    Transaction = 1,
}

/**
 * The base class for block payloads.
 */
abstract class Payload {
    /**
     * The type of payload.
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
