# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.network_info import NetworkInfo


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
        return {k: v for k, v in self.__dict__.items() if v != None}


@dataclass
class ClientOptions:
    """Client options.


        Attributes:
        nodes (List[str]): 
            Array of Node URLs.
        primary_node (str): 
            Node which will be tried first for all requests.
        primary_pow_node (str):
            Node which will be tried first when using remote PoW, even before the primary_node.
        permanode (str):
            Permanode URL.
        ignoreNodeHealth (bool):
            If the node health should be ignored.
        apiTimeout (Duration):
            Timeout for API requests.
        nodeSyncInterval (Duration):
            Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
        remotePowTimeout (Duration):
            Timeout when sending a block that requires remote proof of work.
        tipsInterval (int):
            Tips request interval during PoW in seconds.
        quorum (bool):
            If node quorum is enabled. Will compare the responses from multiple nodes
            and only returns the response if `quorum_threshold`% of the nodes return the same one.
        minQuorumSize (int):
            Minimum amount of nodes required for request when quorum is enabled.
        quorumThreshold (int):
            % of nodes that have to return the same response so it gets accepted.
        userAgent (str):
            The User-Agent header for requests.
        localPow (bool):
            Local proof of work.
        fallbackToLocalPow (bool):
            Fallback to local proof of work if the node doesn't support remote PoW.
        powWorkerCount (int):
            The amount of threads to be used for proof of work.
    """
    primaryNode: Optional[str] = None
    primaryPowNode: Optional[str] = None
    nodes: Optional[List[str]] = None
    permanodes: Optional[List[str]] = None
    ignoreNodeHealth: Optional[bool] = None
    nodeSyncInterval: Optional[Duration] = None
    tipsInterval: Optional[int] = None
    quorum: Optional[bool] = None
    minQuorumSize: Optional[int] = None
    quorumThreshold: Optional[int] = None
    userAgent: Optional[str] = None
    networkInfo: Optional[NetworkInfo] = None
    brokerOptions: Optional[MqttBrokerOptions] = None
    apiTimeout: Optional[Duration] = None
    remotePowTimeout: Optional[Duration] = None
    powWorkerCount: Optional[int] = None
    localPow: Optional[bool] = None
    fallbackToLocalPow: Optional[bool] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'brokerOptions' in config:
            config['brokerOptions'] = config['brokerOptions'].as_dict()

        return config
