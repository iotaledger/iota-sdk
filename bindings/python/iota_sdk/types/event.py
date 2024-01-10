# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum


class WalletEventType(IntEnum):
    """Types of wallet events.

    Attributes:
        LedgerAddressGeneration (0): Nano Ledger has generated an address.
        NewOutput (1): A new output was created.
        SpentOutput (2): An output was spent.
        TransactionInclusion (3): A transaction was included into the ledger.
        TransactionProgress (4): A progress update while submitting a transaction.
    """
    LedgerAddressGeneration = 0
    NewOutput = 1
    SpentOutput = 2
    TransactionInclusion = 3
    TransactionProgress = 4
