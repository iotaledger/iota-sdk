// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountId } from '../..';
import { u16 } from '../../utils/type-aliases';
import { SlotCommitmentId } from '../slot';

enum ContextInputType {
    /**
     * The context input type of a `CommitmentContextInput`.
     */
    COMMITMENT = 0,
    /**
     * The context input kind of a `BlockIssuanceCreditContextInput`.
     */
    BLOCK_ISSUANCE_CREDIT = 1,
    /**
     * The context input kind of a `RewardContextInput`.
     */
    REWARD = 2,
}

abstract class ContextInput {
    type: ContextInputType;

    constructor(type: ContextInputType) {
        this.type = type;
    }
}

/**
 * A Commitment Context Input references a commitment to a certain slot.
 */
class CommitmentContextInput extends ContextInput {
    readonly commitmentId: SlotCommitmentId;

    constructor(commitmentId: SlotCommitmentId) {
        super(ContextInputType.COMMITMENT);
        this.commitmentId = commitmentId;
    }
}

/**
 * A Block Issuance Credit (BIC) Input provides the VM with context for the value of
 * the BIC vector of a specific slot.
 */
class BlockIssuanceCreditContextInput extends ContextInput {
    accountId: AccountId;

    constructor(accountId: AccountId) {
        super(ContextInputType.BLOCK_ISSUANCE_CREDIT);
        this.accountId = accountId;
    }
}

/**
 * A Reward Context Input indicates which transaction Input is claiming Mana rewards.
 */
class RewardContextInput extends ContextInput {
    index: u16;

    constructor(index: u16) {
        super(ContextInputType.REWARD);
        this.index = index;
    }
}

const ContextInputDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: CommitmentContextInput,
            name: ContextInputType.COMMITMENT as any,
        },
        {
            value: BlockIssuanceCreditContextInput,
            name: ContextInputType.BLOCK_ISSUANCE_CREDIT as any,
        },
        { value: RewardContextInput, name: ContextInputType.REWARD as any },
    ],
};

export {
    ContextInputType,
    ContextInput,
    CommitmentContextInput,
    RewardContextInput,
    BlockIssuanceCreditContextInput,
    ContextInputDiscriminator,
};
