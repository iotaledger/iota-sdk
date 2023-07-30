// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../../utils/hex-encoding';
import { Payload, PayloadType } from '../payload';

/**
 * A Tagged Data payload.
 */
class TaggedDataPayload extends Payload {
    /**
     * The tag to use to categorize the data.
     */
    tag: HexEncodedString;
    /**
     * The index data.
     */
    data: HexEncodedString;
    /**
     * @param tag A tag as hex-encoded string.
     * @param data Index data as hex-encoded string.
     */
    constructor(tag: HexEncodedString, data: HexEncodedString) {
        super(PayloadType.TaggedData);
        this.tag = tag;
        this.data = data;
    }
}

export { TaggedDataPayload };
