# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from iota_sdk.types.common import json
from iota_sdk.types.node_info import NodeInfoProtocol


@json
@dataclass
class NetworkInfo:
    """Network and PoW related information.
    """

    protocol_parameters: NodeInfoProtocol
    local_pow: bool
    tips_interval: int
