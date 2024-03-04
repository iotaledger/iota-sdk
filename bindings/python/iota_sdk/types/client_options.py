# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import json
from iota_sdk.types.node_info import ProtocolParameters


@json
@dataclass
class Duration:
    """Time duration.
    """
    secs: int
    nanos: int


@json
@dataclass
class MqttBrokerOptions:
    """Options for the MQTT broker.

        Attributes:
        automatic_disconnect (bool):
            Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not.
        timeout (Duration):
            Sets the timeout used for the MQTT operations.
        use_ws (bool):
            Sets the use_ws used for the MQTT operations.
        port (int):
            Sets the port used for the MQTT operations.
        max_reconnection_attempts (int):
            Sets the maximum number of reconnection attempts. 0 is unlimited.
    """
    automatic_disconnect: Optional[bool] = None
    timeout: Optional[Duration] = None
    use_ws: Optional[bool] = None
    port: Optional[int] = None
    max_reconnection_attempts: Optional[int] = None


@json
@dataclass
class ClientOptions:
    """Client options.

        Attributes:
        primary_nodes (List[str]):
            Nodes which will be tried first for all requests.
        nodes (List[str]):
            Array of Node URLs.
        ignore_node_health (bool):
            If the node health should be ignored.
        node_sync_interval (Duration):
            Interval in which nodes will be checked for their sync status and the network info gets updated.
        quorum (bool):
            If node quorum is enabled. Will compare the responses from multiple nodes
            and only returns the response if `quorum_threshold`% of the nodes return the same one.
        min_quorum_size (int):
            Minimum amount of nodes required for request when quorum is enabled.
        quorum_threshold (int):
            % of nodes that have to return the same response so it gets accepted.
        user_agent (str):
            The User-Agent header for requests.
        broker_options (MqttBrokerOptions):
            Options for the MQTT broker.
        protocol_parameters (ProtocolParameters):
            Protocol parameters.
        api_timeout (Duration):
            Timeout for API requests.
        max_parallel_api_requests (int):
            The maximum parallel API requests.
    """
    primary_nodes: Optional[List[str]] = None
    nodes: Optional[List[str]] = None
    ignore_node_health: Optional[bool] = None
    node_sync_interval: Optional[Duration] = None
    quorum: Optional[bool] = None
    min_quorum_size: Optional[int] = None
    quorum_threshold: Optional[int] = None
    user_agent: Optional[str] = None
    broker_options: Optional[MqttBrokerOptions] = None
    protocol_parameters: Optional[ProtocolParameters] = None
    api_timeout: Optional[Duration] = None
    max_parallel_api_requests: Optional[int] = None
