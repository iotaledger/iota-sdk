# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum


class WalletEventType(IntEnum):
    """Types of wallet events.
    
    Attributes:
        ConsolidationRequired (0): Consolidation is required
        LedgerAddressGeneration (1): Nano Ledger is generating addresses
        NewOutput (2): New output
        SpentOutput (3): Output spent
        TransactionInclusion (4): Transaction was included
        TransactionProgress (5): Transaction is in progress
    """
    ConsolidationRequired = 0,
    LedgerAddressGeneration = 1,
    NewOutput = 2,
    SpentOutput = 3,
    TransactionInclusion = 4,
    TransactionProgress = 5,
