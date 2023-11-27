# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Dict, Optional, Union
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import AccountOutput, BasicOutput, DelegationOutput, FoundryOutput, NftOutput, deserialize_output


@json
@dataclass
class OutputMetadata:
    """Metadata about an output.

    Attributes:
        block_id: The ID of the block in which the output appeared in.
        transaction_id: The ID of the transaction in which the output was created.
        output_index: The index of the output within the corresponding transaction.
        is_spent: Tells if the output is spent in a confirmed transaction or not.
        latest_commitment_id: The current latest commitment id for which the request was made.
        commitment_id_spent: The commitment ID of the slot at which this output was spent.
        transaction_id_spent: The transaction this output was spent with.
        included_commitment_id: The commitment ID at which the output was included into the ledger.
    """
    block_id: HexStr
    transaction_id: HexStr
    output_index: int
    is_spent: bool
    latest_commitment_id: HexStr
    commitment_id_spent: Optional[HexStr] = None
    transaction_id_spent: Optional[HexStr] = None
    included_commitment_id: Optional[HexStr] = None


@json
@dataclass
class OutputWithMetadata:
    """An output with its metadata.

    Attributes:
        metadata: The `OutputMetadata` object that belongs to `output`.
        output: An `Output` object.
    """

    metadata: OutputMetadata
    output: Union[AccountOutput, FoundryOutput,
                  NftOutput, BasicOutput, DelegationOutput] = field(metadata=config(
                      decoder=deserialize_output
                  ))

    @classmethod
    def from_dict(cls, data_dict: Dict) -> OutputWithMetadata:
        """Creates an output with metadata instance from the dict object.
        """
        obj = cls.__new__(cls)
        super(OutputWithMetadata, obj).__init__()
        for k, v in data_dict.items():
            setattr(obj, k, v)
        return obj

    def as_dict(self):
        """Returns a dictionary representation of OutputWithMetadata, with the fields metadata and output.
        """
        d = {}

        d['metadata'] = self.metadata.__dict__
        d['output'] = self.output.as_dict()

        return d
