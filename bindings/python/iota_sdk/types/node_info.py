# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr


@dataclass
class NodeInfoMilestone:
    """Milestone info.

    Attributes:
        index: The milestone index.
        timestamp: The milestone timestamp.
        milestoneId: The milestone ID.
    """
    index: int
    timestamp: Optional[int] = None
    milestoneId: Optional[HexStr] = None


@dataclass
class NodeInfoStatus:
    """Node status.

    Attributes:
        isHealthy: Whether the node is healthy.
        latestMilestone: The latest milestone info.
        confirmedMilestone: The latest confirmed milestone info.
        pruningIndex: The pruning index of the node.
    """
    isHealthy: bool
    latestMilestone: NodeInfoMilestone
    confirmedMilestone: NodeInfoMilestone
    pruningIndex: int


@dataclass
class NodeInfoMetrics:
    """Node metrics.

    Attributes:
        blocksPerSecond: The blocks per second gossiped in the network.
        referencedBlocksPerSecond: The referenced blocks per second in the network.
        referencedRate: The percentage of blocks that get referenced.
    """
    blocksPerSecond: float
    referencedBlocksPerSecond: float
    referencedRate: float


@dataclass
class RentStructure:
    """Rent structure for the storage deposit.

    Attributes:
        vByteCost: The cost of base coin per virtual byte.
        vByteFactorData: The weight factor used for key fields in the outputs.
        vByteFactorKey: The weight factor used for data fields in the outputs.
    """
    vByteCost: int
    vByteFactorData: int
    vByteFactorKey: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        res = {k: v for k, v in self.__dict__.items() if v is not None}
        return res


@dataclass
class NodeInfoProtocol:
    """Protocol info.

    Attributes:
        networkName: The human friendly name of the network.
        bech32Hrp: The HRP prefix used for Bech32 addresses in the network.
        tokenSupply: TokenSupply defines the current token supply on the network.
        version: The version of the protocol running.
        belowMaxDepth: The below max depth parameter of the network.
        minPowScore: The minimum pow score of the network.
        rentStructure: The rent structure used by given node/network.
    """
    networkName: str
    bech32Hrp: str
    tokenSupply: str
    version: int
    belowMaxDepth: int
    minPowScore: float
    rentStructure: RentStructure

    def as_dict(self):
        """Converts this object to a dict.
        """
        res = {k: v for k, v in self.__dict__.items() if v is not None}
        if res["rentStructure"]:
            res["rentStructure"] = res["rentStructure"].as_dict()
        return res


@dataclass
class PendingProtocolParameter:
    """Pending protocol parameters.

    Attributes:
        type: Type of change.
        targetMilestoneIndex: Milestone index at which the new protocol parameters become active.
        protocolVersion: The new protocol version.
        params: The new protocol parameters.
    """
    type: int
    targetMilestoneIndex: int
    protocolVersion: int
    params: str


@dataclass
class NodeInfoBaseToken:
    """The base coin info.

    Attributes:
        name: Name of the base coin.
        tickerSymbol: Base coin ticker symbol.
        unit: Base coin unit.
        decimals: Number of decimals.
        useMetricPrefix: Whether the coin uses a metric prefix.
        subunit: Base coin subunit.
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

    Attributes:
        name: The name of the node (e.g. Hornet).
        version: The version of the node.
        status: The status of the node.
        metrics: Some node metrics.
        supportedProtocolVersions: Supported protocol versions by the node.
        protocol: Information about the running protocol.
        pendingProtocolParameters: A list of pending (not yet active) protocol parameters.
        baseToken: Information about the base token.
        features: List of features supported by the node.
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

    Attributes:
        nodeInfo: A NodeInfo object.
        url: The URL of the node.
    """
    nodeInfo: NodeInfo
    url: str
