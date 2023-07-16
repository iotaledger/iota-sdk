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
        index (int): Index of the milestone.
        timestamp (int, optional): Timestamp of the milestone.
        milestoneId (HexStr, optional): ID of the milestone.
    """

    index: int
    timestamp: Optional[int] = None
    milestoneId: Optional[HexStr] = None


@dataclass
class NodeInfoStatus:
    """Node status.

    Attributes:
        isHealthy (bool): Whether the node is healthy
        latestMilestone (NodeInfoMilestone): Latest milestone info
        confirmedMilestone (NodeInfoMilestone): Confirmed milestone info
        pruningIndex (int): Pruning index of the node
    """

    isHealthy: bool
    latestMilestone: NodeInfoMilestone
    confirmedMilestone: NodeInfoMilestone
    pruningIndex: int


@dataclass
class NodeInfoMetrics:
    """Node metrics.

    Attributes:
        blocksPerSecond (float): Blocks per second
        referencedBlocksPerSecond (float): Referenced blocks per second
        referencedRate (float): Referenced rate
    """

    blocksPerSecond: float
    referencedBlocksPerSecond: float
    referencedRate: float


@dataclass
class RentStructure:
    """Rent structure for the storage deposit.

    Attributes:
        vByteCost (int): Virtual byte cost
        vByteFactorData (int): Virtual byte factor data
        vByteFactorKey (int): Virtual byte factor key
    """
    vByteCost: int
    vByteFactorData: int
    vByteFactorKey: int


@dataclass
class NodeInfoProtocol:
    """Protocol info.

    Attributes:
        networkName (str): Network name
        bech32Hrp (str): Bech32 HRP (human readable part)
        tokenSupply (str): Token supply
        version (int): Protocol version
        minPowScore (float): The Minimum PoW score
        rentStructure (RentStructure): The rent structure
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

    Attributes:
        type (int): Type of change
        targetMilestoneIndex (int): Milestone index at which the new protocol parameters become effective
        protocolVersion (int): New protocol version
        params (str): New protocol parameters
    """

    type: int
    targetMilestoneIndex: int
    protocolVersion: int
    params: str


@dataclass
class NodeInfoBaseToken:
    """The base coin info.

    Attributes:
        name (str): Name of the base coin
        tickerSymbol (str): Base coin ticker symbol
        unit (str): Base coin unit
        decimals (int): Number of decimals
        useMetricPrefix (bool): Whether the coin uses a metric prefix
        subunit (str, optional): Base coin subunit
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
        name (str): Name of the node
        version (str): Version of the node
        status (NodeInfoStatus): Node status
        metrics (NodeInfoMetrics): Node metrics
        supportedProtocolVersions (List[int]): List of supported protocol versions
        protocol (NodeInfoProtocol): Protocol info
        pendingProtocolParameters (List[PendingProtocolParameter]): List of pending protocol parameters
        baseToken (NodeInfoBaseToken): Base coin info
        features (List[str]): List of node features
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
        nodeInfo (NodeInfo): Node info
        url (str): URL of the node
    """
    nodeInfo: NodeInfo
    url: str
