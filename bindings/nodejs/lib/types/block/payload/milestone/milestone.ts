// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import {
    MilestoneOption,
    MilestoneOptionDiscriminator,
} from '../../../models/milestone_options';
import { HexEncodedString } from '../../../utils/hex-encoding';
import { Ed25519Signature } from '../../signature';
import { Payload, PayloadType } from '../payload';

/**
 * A milestone payload.
 */
class MilestonePayload extends Payload {
    /**
     * The index name.
     */
    index!: number;
    /**
     * The timestamp of the milestone.
     */
    timestamp!: number;
    /**
     * The protocol version.
     */
    protocolVersion!: number;
    /**
     * The id of the previous milestone.
     */
    previousMilestoneId!: HexEncodedString;
    /**
     * The parents where this milestone attaches to.
     */
    parents!: HexEncodedString[];
    /**
     * The Merkle tree hash of all blocks confirmed by this milestone.
     */
    inclusionMerkleRoot!: HexEncodedString;
    /**
     * The Merkle tree hash of all blocks applied by this milestone.
     */
    appliedMerkleRoot!: HexEncodedString;
    /**
     * The metadata.
     */
    metadata?: HexEncodedString;
    /**
     * The milestone options.
     */
    @Type(() => MilestoneOption, {
        discriminator: MilestoneOptionDiscriminator,
    })
    options?: MilestoneOption[];
    /**
     * The signatures.
     */
    @Type(() => Ed25519Signature)
    signatures!: Ed25519Signature[];

    constructor() {
        super(PayloadType.Milestone);
    }
}

export { MilestonePayload };
