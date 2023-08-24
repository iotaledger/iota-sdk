# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr, json


@json
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


@json
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


@json
@dataclass
class NodeInfoMetrics:
    """Node metrics.

    Attributes:
        blocks_per_second: The blocks per second gossiped in the network.
        referenced_blocks_per_second: The referenced blocks per second in the network.
        referenced_rate: The percentage of blocks that get referenced.
    """
    blocks_per_second: float
    referenced_block_per_second: float
    referenced_rate: float


@json
@dataclass
class RentStructure:
    """Rent structure for the storage deposit.

    Attributes:
        v_byte_cost: The cost of base coin per virtual byte.
        v_byte_factor_data: The weight factor used for key fields in the outputs.
        v_byte_factor_key: The weight factor used for data fields in the outputs.
    """
    v_byte_cost: int
    v_byte_factor_data: int
    v_byte_factor_key: int


@json
@dataclass
class NodeInfoProtocol:
    """Protocol info.

    Attributes:
        networkName: The human friendly name of the network.
        bech32Hrp: The HRP prefix used for Bech32 addresses in the network.
        tokenSupply: TokenSupply defines the current token supply on the network.
        version: The version of the protocol running.
        minPowScore: The minimum pow score of the network.
        rentStructure: The rent structure used by given node/network.
    """
    networkName: str
    bech32Hrp: str
    tokenSupply: str
    version: int
    minPowScore: float
    rentStructure: RentStructure


@json
@dataclass
class PendingProtocolParameter:
    """Pending protocol parameters.

    Attributes:
        type: Type of change.
        target_milestone_index: Milestone index at which the new protocol parameters become active.
        protocol_version: The new protocol version.
        params: The new protocol parameters.
    """
    type: int
    target_milestone_index: int
    protocol_version: int
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


@json
@dataclass
class NodeInfo:
    """Response from the /info endpoint.

    Attributes:
        name: The name of the node (e.g. Hornet).
        version: The version of the node.
        status: The status of the node.
        metrics: Some node metrics.
        supported_protocol_versions: Supported protocol versions by the ndoe.
        protocol: Information about the running protocol.
        pending_protocol_parameters: A list of pending (not yet active) protocol parameters.
        base_token: Information about the base token.
        features: List of features supported by the node.
    """

    name: str
    version: str
    status: NodeInfoStatus
    metrics: NodeInfoMetrics
    supported_protocol_version: List[int]
    protocol: NodeInfoProtocol
    pending_protocol_parameters: List[PendingProtocolParameter]
    base_token: NodeInfoBaseToken
    features: List[str]


@json
@dataclass
class NodeInfoWrapper:
    """NodeInfo wrapper which contains the node info and the url from the node.

    Attributes:
        node_info: A NodeInfo object.
        url: The URL of the node.
    """
    node_info: NodeInfo
    url: str
