# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Dict, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.common import SlotIndex, json
from iota_sdk.types.output import Output, deserialize_output
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.slot import SlotCommitmentId
from iota_sdk.types.transaction_id import TransactionId


@json
@dataclass
class OutputMetadata:
    """Metadata about an output.
    Response of GET /api/core/v3/outputs/{outputId}/metadata.

    Attributes:
        output_id: The ID of the output.
        block_id: The ID of the block in which the output appeared in.
        included: Metadata of the output if it is included in the ledger.
        latest_commitment_id: Latest commitment ID of the node.
        spent: Metadata of the output if it is marked as spent in the ledger.
    """
    output_id: OutputId
    block_id: BlockId
    included: OutputInclusionMetadata
    latest_commitment_id: SlotCommitmentId
    spent: Optional[OutputConsumptionMetadata] = None


@json
@dataclass
class OutputWithMetadata:
    """An output and its metadata.

    Attributes:
        output: An `Output` object.
        metadata: The `OutputMetadata` object that belongs to `output`.
    """
    output: Output = field(metadata=config(
        decoder=deserialize_output
    ))
    metadata: OutputMetadata

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

        d['output'] = self.output.as_dict()
        d['metadata'] = self.metadata.to_dict()

        return d


@json
@dataclass
class OutputInclusionMetadata:
    """Metadata about a created (unspent) output.

    Attributes:
        slot: Slot in which the output was included.
        transaction_id: Transaction ID that created the output.
        commitment_id: Commitment ID that includes the creation of the output.
    """
    slot: SlotIndex
    transaction_id: TransactionId
    commitment_id: Optional[SlotCommitmentId] = None


@json
@dataclass
class OutputConsumptionMetadata:
    """Metadata about a consumed (spent) output.

    Attributes:
        slot: Slot in which the output was spent.
        transaction_id: Transaction ID that spent the output.
        commitment_id: Commitment ID that includes the spending of the output.
    """
    slot: SlotIndex
    transaction_id: TransactionId
    commitment_id: Optional[SlotCommitmentId] = None
