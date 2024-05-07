// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { AccountId } from '../block';
import { SignedTransactionPayload } from '../block/payload/signed_transaction';
import { InputSigningData } from '../client';
import { HexEncodedString, NumericString } from '../utils';

/** The signed transaction with inputs data */
export class SignedTransactionData {
    /** A signed transaction payload. */
    @Type(() => SignedTransactionPayload)
    payload!: SignedTransactionPayload;
    /** Signed inputs data. */
    @Type(() => InputSigningData)
    inputsData!: InputSigningData;
    /** Mana rewards by input. */
    manaRewards?: { [outputId: HexEncodedString]: NumericString };
    /** The block issuer id from which the BIC for the block containing this transaction should be burned. */
    issuerId?: AccountId;
}
