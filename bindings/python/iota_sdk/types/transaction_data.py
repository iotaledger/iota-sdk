# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.address import Address
from iota_sdk.types.output import Output, OutputMetadata
from iota_sdk.types.payload import RegularTransactionEssence, TransactionPayload
from iota_sdk.types.signature import Bip44


@dataclass
class InputSigningData:
    """Data for transaction inputs for signing and ordering of unlock blocks.

    Attributes:
        output: The output.
        outputMetadata: The output metadata.
        chain: The BIP44 chain for the address to unlock the output.
    """
    output: Output
    outputMetadata: OutputMetadata
    chain: Optional[Bip44] = None


@dataclass
class RemainderData:
    """Data for a remainder output, used for ledger nano.

    Attributes:
        output: The output.
        address: The remainder address.
        chain: The BIP44 chain for the remainder address.
    """
    output: Output
    address: Address
    chain: Optional[Bip44] = None


@dataclass
class PreparedTransactionData:
    """Helper class for offline signing.

    Attributes:
        essence: The transaction essence.
        inputsData: Data about the inputs which is required for signing.
        remainder: Data about a remainder.
    """
    essence: RegularTransactionEssence
    inputsData: List[InputSigningData]
    remainder: Optional[RemainderData] = None


@dataclass
class SignedTransactionData:
    """Helper class for offline signing.

    Attributes:
        transactionPayload: The transaction payload.
        inputsData: Data about the inputs consumed in the transaction.
    """
    transactionPayload: TransactionPayload
    inputsData: List[InputSigningData]
