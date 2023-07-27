// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { HexEncodedString } from '../../../utils/hex-encoding';
import { Payload, PayloadType } from '../payload';

/**
 * Tagged data payload.
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
    constructor(tag: HexEncodedString, data: HexEncodedString) {
        super(PayloadType.TaggedData);
        this.tag = tag;
        this.data = data;
    }
}

export { TaggedDataPayload };
