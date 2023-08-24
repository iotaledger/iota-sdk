# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, asdict
from typing import Optional, List
from iota_sdk.types.address import Ed25519Address, AliasAddress, NFTAddress
from iota_sdk.types.output import BasicOutput, AliasOutput, FoundryOutput, NftOutput, OutputMetadata
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
    output: AliasOutput | FoundryOutput | NftOutput | BasicOutput
    output_metadata: OutputMetadata
    chain: Optional[Bip44] = None

    def as_dict(self):

        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['output'] = config['output'].to_dict()
        config['outputMetadata'] = config['outputMetadata'].to_dict()
        if 'chain' in config:
            config['chain'] = asdict(config['chain'])

        return config


@dataclass
class RemainderData:
    """Data for a remainder output, used for ledger nano.

    Attributes:
        output: The output.
        address: The remainder address.
        chain: The BIP44 chain for the remainder address.
    """
    output: AliasOutput | FoundryOutput | NftOutput | BasicOutput
    address: Ed25519Address | AliasAddress | NFTAddress
    chain: Optional[Bip44] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['output'] = config['output'].to_dict()
        config['address'] = config['address'].to_dict()
        if 'chain' in config:
            config['chain'] = asdict(config['chain'])

        return config


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

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['essence'] = config['essence'].to_dict()

        config['inputsData'] = list(map(
            lambda x: x.to_dict(), config['inputsData']))

        if 'remainder' in config:
            config['remainder'] = config['remainder'].to_dict()

        return config


@dataclass_j    _
@dataclass
class SignedTransactionData:
    """Helper class for offline signing.

    Attributes:
        transaction_payload: The transaction payload.
        inputsData: Data about the inputs consumed in the transaction.
    """
    transactionPayload: TransactionPayload
    inputsData: List[InputSigningData]

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['transactionPayload'] = config['transactionPayload'].to_dict()

        config['inputsData'] = list(map(
            lambda x: x.to_dict(), config['inputsData']))

        return config
