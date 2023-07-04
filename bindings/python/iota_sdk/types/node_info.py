# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr


@dataclass
class NodeInfoMilestone:
    """Milestone info.
    """

    index: int
    timestamp: Optional[int] = None
    milestoneId: Optional[HexStr] = None


@dataclass
class NodeInfoStatus:
    """Node status.
    """

    isHealthy: bool
    latestMilestone: NodeInfoMilestone
    confirmedMilestone: NodeInfoMilestone
    pruningIndex: int


@dataclass
class NodeInfoMetrics:
    """Node metrics.
    """

    blocksPerSecond: float
    referencedBlocksPerSecond: float
    referencedRate: float


@dataclass
class RentStructure:
    """Rent structure for the storage deposit.
    """
    vByteCost: int
    vByteFactorData: int
    vByteFactorKey: int


@dataclass
class NodeInfoProtocol:
    """Protocol info.
    """

    networkName: str
    bech32Hrp: str
    tokenSupply: str
    version: int
    minPowScore: float
    rentStructure: RentStructure


@dataclass
class PendingProtocolParameter:
    """Pending protocol parameters.
    """

    type: int
    targetMilestoneIndex: int
    protocolVersion: int
    params: str


@dataclass
class NodeInfoBaseToken:
    """The base token info.
    """

    name: str
    tickerSymbol: str
    unit: str
    decimals: int
    useMetricPrefix: bool
    subunit: Optional[str] = None


@dataclass
class NodeInfo:
    """Response from the /info endpoint.
    """

    name: str
    version: str
    status: NodeInfoStatus
    metrics: NodeInfoMetrics
    supportedProtocolVersions: List[int]
    protocol: NodeInfoProtocol
    pendingProtocolParameters: List[PendingProtocolParameter]
    baseToken: NodeInfoBaseToken
    features: List[str]


@dataclass
class NodeInfoWrapper:
    """NodeInfo wrapper which contains the node info and the url from the node.
    """
    nodeInfo: NodeInfo
    url: str
