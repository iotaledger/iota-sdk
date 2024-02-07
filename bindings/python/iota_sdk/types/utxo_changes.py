# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List

from dataclasses import dataclass
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.output import Output


@json
@dataclass
class UtxoChangesResponse():
    """All UTXO changes that happened at a specific slot.
    Response of
    - GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes

    Arguments:
        commitment_id: The commitment ID of the requested slot that contains the changes. Hex-encoded with 0x prefix.
        created_outputs: The created outputs of the given slot.
        consumed_outputs: The consumed outputs of the given slot.
    """
    commitment_id: HexStr
    created_outputs: List[OutputId]
    consumed_outputs: List[OutputId]


@json
@dataclass
class OutputWithId():
    """An Output and its ID.

    Arguments:
        output_id: OutputId,
        output: Output,
    """
    output_id: OutputId
    output: Output


@json
@dataclass
class UtxoChangesFullResponse():
    """All full UTXO changes that happened at a specific slot.
    Response of
    - GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
    - GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full

    Arguments:
        commitment_id: The commitment ID of the requested slot that contains the changes. Hex-encoded with 0x prefix.
        created_outputs: The created outputs of the given slot.
        consumed_outputs: The consumed outputs of the given slot.
    """
    commitment_id: HexStr
    created_outputs: List[OutputWithId]
    consumed_outputs: List[OutputWithId]
