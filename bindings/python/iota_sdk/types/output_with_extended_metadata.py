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
class OutputWithExtendedMetadata:
    """Output with extended metadata.

    Attributes:
        output: The output object itself.
        metadata: With the output corresponding metadata.
        output_id: With the output data corresponding output ID.
        output_id_proof: The output ID proof.
        network_id: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
    """
    output: Output
    metadata: OutputMetadata
    output_id: OutputId
    output_id_proof: OutputIdProof
    network_id: str
    remainder: bool
