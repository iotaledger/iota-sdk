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
    """Output data.

    Attributes:
        output_id: With the output data corresponding output ID.
        metadata: With the output corresponding metadata.
        output: The output object itself.
        output_id_proof: The output ID proof.
        address: The address associated with the output.
        network_id: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
        chain: A list of chain state indexes.
    """
    output_id: OutputId
    metadata: OutputMetadata
    output: Output
    output_id_proof: OutputIdProof
    network_id: str
    remainder: bool
