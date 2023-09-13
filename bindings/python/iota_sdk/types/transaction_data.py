# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.address import Ed25519Address, AccountAddress, NFTAddress
from iota_sdk.types.output import BasicOutput, AccountOutput, FoundryOutput, NftOutput, OutputMetadata
from iota_sdk.types.payload import RegularTransactionEssence, TransactionPayload
from iota_sdk.types.signature import Bip44
from iota_sdk.types.common import json


@json
@dataclass
class InputSigningData:
    """Data for transaction inputs for signing and ordering of unlock blocks.

    Attributes:
        output: The output.
        output_metadata: The output metadata.
        chain: The BIP44 chain for the address to unlock the output.
    """
    output: AccountOutput | FoundryOutput | NftOutput | BasicOutput
    output_metadata: OutputMetadata
    chain: Optional[Bip44] = None


@json
@dataclass
class RemainderData:
    """Data for a remainder output, used for ledger nano.

    Attributes:
        output: The output.
        address: The remainder address.
        chain: The BIP44 chain for the remainder address.
    """
    output: AccountOutput | FoundryOutput | NftOutput | BasicOutput
    address: Ed25519Address | AccountAddress | NFTAddress
    chain: Optional[Bip44] = None


@json
@dataclass
class PreparedTransactionData:
    """Helper class for offline signing.

    Attributes:
        essence: The transaction essence.
        inputs_data: Data about the inputs which is required for signing.
        remainder: Data about a remainder.
    """
    essence: RegularTransactionEssence
    inputs_data: List[InputSigningData]
    remainder: Optional[RemainderData] = None


@json
@dataclass
class SignedTransactionData:
    """Helper class for offline signing.

    Attributes:
        transaction_payload: The transaction payload.
        inputs_data: Data about the inputs consumed in the transaction.
    """
    transaction_payload: TransactionPayload
    inputs_data: List[InputSigningData]
