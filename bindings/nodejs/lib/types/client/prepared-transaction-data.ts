// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../block/address';
import { Output, OutputDiscriminator } from '../block/output/output';
import {
    TransactionEssence,
    TransactionEssenceDiscriminator,
} from '../block/payload/transaction/essence';
import { IOutputMetadataResponse } from '../models/api';
import { Bip44 } from '../secret_manager';

/**
 * Helper struct for offline signing.
 */
export class PreparedTransactionData {
    /**
     * Transaction essence
     */
    @Type(() => TransactionEssence, {
        discriminator: TransactionEssenceDiscriminator,
    })
    essence!: TransactionEssence;
    /**
     * Required address information for signing
     */
    inputsData!: InputSigningData[];
    /**
     * Optional remainder output information
     */
    remainder?: Remainder;
}

/**
 * Data for transaction inputs for signing and ordering of unlock blocks.
 */
export class InputSigningData {
    /**
     * The output
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /**
     * The output metadata
     */
    outputMetadata!: IOutputMetadataResponse;
    /**
     * The chain derived from seed, only for ed25519 addresses
     */
    chain?: Bip44;
}

/**
 * Data for a remainder output, used for Ledger Nano.
 */
export class Remainder {
    /**
     * The remainder output.
     */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /**
     * The chain derived from seed, for the remainder addresses.
     */
    chain?: Bip44;
    /**
     * The remainder address.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    address!: Address;
}
