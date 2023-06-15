// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Temp solution for not double parsing JSON
import { HexEncodedString } from '../../../utils';

/**
 * All of the essence types.
 */
enum TransactionEssenceType {
    Regular = 1,
}

abstract class TransactionEssence {
    private type: TransactionEssenceType;

    constructor(type: TransactionEssenceType) {
        this.type = type;
    }

    /**
     * The type of essence.
     */
    getType(): TransactionEssenceType {
        return this.type;
    }
}

/**
 * RegularTransactionEssence transaction essence.
 */
class RegularTransactionEssence extends TransactionEssence {
    /**
     * The public key.
     */
    publicKey: HexEncodedString;
    /**
     * The transactionessence.
     */
    transactionEssence: HexEncodedString;

    constructor(
        publicKey: HexEncodedString,
        transactionEssence: HexEncodedString,
    ) {
        super(TransactionEssenceType.Regular);
        this.publicKey = publicKey;
        this.transactionEssence = transactionEssence;
    }
}

const TransactionEssenceDiscriminator = {
    property: 'type',
    subTypes: [
        {
            value: RegularTransactionEssence,
            name: TransactionEssenceType.Regular as any,
        },
    ],
};

export {
    TransactionEssenceDiscriminator,
    TransactionEssence,
    TransactionEssenceType,
    RegularTransactionEssence,
};
