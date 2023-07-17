# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from iota_sdk.types.node_info import NodeInfoProtocol


@dataclass
class NetworkInfo:
    """Network and PoW related information.
    """

    protocolParameters: NodeInfoProtocol
    localPow: bool
    tipsInterval: int
