# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from dataclasses_json import config
from typing import Dict, Optional, Union
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import AccountOutput, BasicOutput, DelegationOutput, FoundryOutput, NftOutput, outputs_from_dicts


@json
@dataclass
class OutputMetadata:
    """Metadata about an output.

    Attributes:
        block_id: The ID of the block in which the output appeared in.
        transaction_id: The ID of the transaction in which the output was created.
        output_index: The index of the output within the corresponding transaction.
        is_spent: Whether the output is already spent.
        milestone_index_booked: The index of the milestone which booked/created the output.
        milestone_timestamp_booked: The timestamp of the milestone which booked/created the output.
        ledger_index: The current ledger index.
        milestone_index_spent: The index of the milestone which spent the output.
        milestone_timestamp_spent: The timestamp of the milestone which spent the output.
        transaction_id_spent: The ID of the transaction that spent the output.
    """
    block_id: HexStr
    transaction_id: HexStr
    output_index: int
    is_spent: bool
    milestone_index_booked: int
    milestone_timestamp_booked: int
    ledger_index: int
    milestone_index_spent: Optional[int] = None
    milestone_timestamp_spent: Optional[int] = None
    transaction_id_spent: Optional[HexStr] = None


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
                      decoder=outputs_from_dicts
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
        config = {}

        config['metadata'] = self.metadata.__dict__
        config['output'] = self.output.as_dict()

        return config
