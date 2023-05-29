// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Address } from '../block/address';
import type { HexEncodedString } from '../utils/hexEncodedTypes';
/**
 * The migrated funds for receipts.
 */
export interface IMigratedFunds {
    /**
     * The tail transaction hash of the migration bundle.
     */
    tailTransactionHash: HexEncodedString;
    /**
     * The target address of the migrated funds.
     */
    address: Address;
    /**
     * The amount of the deposit.
     */
    deposit: string;
}
