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
    """
    automaticDisconnect: Optional[bool] = None
    timeout: Optional[int] = None
    useWs: Optional[bool] = None
    port: Optional[int] = None
    maxReconnectionAttempts: Optional[int] = None

    def as_dict(self):
        return {k: v for k, v in self.__dict__.items() if v != None}


@dataclass
class ClientOptions:
    """Client options.
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
    networkInfo: Optional[NetworkInfo] = None
    brokerOptions: Optional[MqttBrokerOptions] = None
    apiTimeout: Optional[Duration] = None
    remotePowTimeout: Optional[Duration] = None
    powWorkerCount: Optional[int] = None
    localPow: Optional[bool] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'brokerOptions' in config:
            config['brokerOptions'] = config['brokerOptions'].as_dict()

        return config
