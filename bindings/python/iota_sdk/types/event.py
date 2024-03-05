# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum


class WalletEventType(IntEnum):
    """Types of wallet events.

    Attributes:
        SelectingInputs (0): Performing input selection.
        GeneratingRemainderDepositAddress (1): Generating remainder value deposit address.
        PreparedTransaction (2): Prepared transaction.
        SigningTransaction (3): Signing the transaction.
        PreparedTransactionSigningHash (4): Prepared transaction signing hash hex encoded, required for blindsigning with a Ledger Nano.
        PreparedBlockSigningInput (5): Prepared block signing input, required for blindsigning with a Ledger Nano.
        Broadcasting (6): Broadcasting.
    """
    SelectingInputs = 0
    GeneratingRemainderDepositAddress = 1
    PreparedTransaction = 2
    SigningTransaction = 3
    PreparedTransactionSigningHash = 4
    PreparedBlockSigningInput = 5
    Broadcasting = 6
