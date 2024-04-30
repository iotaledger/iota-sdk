# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from iota_sdk.types.common import json
from iota_sdk.types.output import Output
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.output_id_proof import OutputIdProof
from iota_sdk.types.output_metadata import OutputMetadata


@json
@dataclass
class OutputData:
    """An output with additional data.

    Attributes:
        output: The output itself.
        metadata: The metadata of the output.
        output_id_proof: The output ID proof.
        output_id: The corresponding output ID.
        network_id: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
    """
    output: Output
    metadata: OutputMetadata
    output_id_proof: OutputIdProof
    output_id: OutputId
    network_id: str
    remainder: bool
