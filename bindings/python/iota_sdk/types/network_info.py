# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from iota_sdk.types.common import json
from iota_sdk.types.node_info import ProtocolParameters


@json
@dataclass
class NetworkInfo:
    """Network related information.
    """
    protocol_parameters: ProtocolParameters
