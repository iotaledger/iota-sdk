# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum


class WalletEventType(IntEnum):
    """Types of wallet events.
    
    Attributes:
        ConsolidationRequired (0): Consolidation is required.
        LedgerAddressGeneration (1): Nano Ledger is generating addresses. TODO.
        NewOutput (2): A new output was created.
        SpentOutput (3): An output was spent.
        TransactionInclusion (4): A transaction was included into the ledger.
        TransactionProgress (5): TODO.
    """
    ConsolidationRequired = 0,
    LedgerAddressGeneration = 1,
    NewOutput = 2,
    SpentOutput = 3,
    TransactionInclusion = 4,
    TransactionProgress = 5,
