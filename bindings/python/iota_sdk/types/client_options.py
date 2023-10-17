# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.node_info import NodeInfoProtocol


@dataclass
class Duration:
    """Time duration.
    """
    secs: int
    nanos: int


@dataclass
class MqttBrokerOptions:
    """Options for the MQTT broker.

        Attributes:
        automaticDisconnect (bool):
            Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not.
        timeout (Duration):
            Sets the timeout used for the MQTT operations.
        useWs (bool):
            Sets the use_ws used for the MQTT operations.
        port (int):
            Sets the port used for the MQTT operations.
        maxReconnectionAttempts (int):
            Sets the maximum number of reconnection attempts. 0 is unlimited.
    """
    automaticDisconnect: Optional[bool] = None
    timeout: Optional[Duration] = None
    useWs: Optional[bool] = None
    port: Optional[int] = None
    maxReconnectionAttempts: Optional[int] = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class ClientOptions:
    """Client options.


        Attributes:
        primary_node (str):
            Node which will be tried first for all requests.
        primary_pow_node (str):
            Node which will be tried first when using remote PoW, even before the primary_node.
        nodes (List[str]):
            Array of Node URLs.
        permanode (str):
            Permanode URL.
        ignoreNodeHealth (bool):
            If the node health should be ignored.
        nodeSyncInterval (Duration):
            Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
        quorum (bool):
            If node quorum is enabled. Will compare the responses from multiple nodes
            and only returns the response if `quorum_threshold`% of the nodes return the same one.
        minQuorumSize (int):
            Minimum amount of nodes required for request when quorum is enabled.
        quorumThreshold (int):
            % of nodes that have to return the same response so it gets accepted.
        userAgent (str):
            The User-Agent header for requests.
        brokerOptions (MqttBrokerOptions):
            Options for the MQTT broker.
        protocolParameters (NodeInfoProtocol):
            Protocol parameters.
        localPow (bool):
            Local proof of work.
        fallbackToLocalPow (bool):
            Fallback to local proof of work if the node doesn't support remote PoW.
        tipsInterval (int):
            Tips request interval during PoW in seconds.
        apiTimeout (Duration):
            Timeout for API requests.
        remotePowTimeout (Duration):
            Timeout when sending a block that requires remote proof of work.
        powWorkerCount (int):
            The amount of threads to be used for proof of work.
        maxParallelApiRequests (int):
            The maximum parallel API requests.
    """
    primaryNode: Optional[str] = None
    primaryPowNode: Optional[str] = None
    nodes: Optional[List[str]] = None
    permanodes: Optional[List[str]] = None
    ignoreNodeHealth: Optional[bool] = None
    nodeSyncInterval: Optional[Duration] = None
    quorum: Optional[bool] = None
    minQuorumSize: Optional[int] = None
    quorumThreshold: Optional[int] = None
    userAgent: Optional[str] = None
    brokerOptions: Optional[MqttBrokerOptions] = None
    protocolParameters: Optional[NodeInfoProtocol] = None
    localPow: Optional[bool] = None
    fallbackToLocalPow: Optional[bool] = None
    tipsInterval: Optional[int] = None
    apiTimeout: Optional[Duration] = None
    remotePowTimeout: Optional[Duration] = None
    powWorkerCount: Optional[int] = None
    maxParallelApiRequests: Optional[int] = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'brokerOptions' in config:
            config['brokerOptions'] = config['brokerOptions'].as_dict()

        return config
