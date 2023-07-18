// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../block/address';
import type { HexEncodedString } from '../utils/hex-encoding';
/**
 * The migrated funds for receipts.
 */
export class MigratedFunds {
    /**
     * The tail transaction hash of the migration bundle.
     */
    tailTransactionHash!: HexEncodedString;
    /**
     * The target address of the migrated funds.
     */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    address!: Address;
    /**
     * The amount of the deposit.
     */
    deposit!: string;
}
