# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import IntEnum
from typing import Dict, Optional, List
from iota_sdk.types.common import HexStr
from iota_sdk.types.feature import Feature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.token_scheme import TokenScheme
from iota_sdk.types.unlock_condition import UnlockCondition


class OutputType(IntEnum):
    Treasury = 2
    Basic = 3
    Alias = 4
    Foundry = 5
    Nft = 6


@dataclass
class Output():
    type: int
    amount: str
    unlockConditions: List[UnlockCondition]
    aliasId: Optional[HexStr] = None
    nftId: Optional[HexStr] = None
    stateIndex: Optional[int] = None
    stateMetadata: Optional[HexStr] = None
    foundryCounter: Optional[int] = None
    features: Optional[List[Feature]] = None
    nativeTokens: Optional[List[NativeToken]] = None
    immutableFeatures: Optional[List[Feature]] = None
    serialNumber: Optional[int] = None
    tokenScheme: Optional[TokenScheme] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config['unlockConditions'] = list(map(
            lambda x: x.as_dict(), config['unlockConditions']))
        if 'nativeTokens' in config:
            config['nativeTokens'] = list(map(
                lambda x: x.__dict__, config['nativeTokens']))
        if 'features' in config:
            config['features'] = list(map(
                lambda x: x.as_dict(), config['features']))
        if 'immutableFeatures' in config:
            config['immutableFeatures'] = list(map(
                lambda x: x.as_dict(), config['immutableFeatures']))
        if 'tokenScheme' in config:
            config['tokenScheme'] = config['tokenScheme'].__dict__

        return config


@dataclass
class OutputMetadata:
    """Metadata about an output.
    """

    blockId: HexStr
    transactionId: HexStr
    outputIndex: int
    isSpent: bool
    milestoneIndexBooked: int
    milestoneTimestampBooked: int
    ledgerIndex: int
    milestoneIndexSpent: Optional[int] = None
    milestoneTimestampSpent: Optional[int] = None
    transactionIdSpent: Optional[HexStr] = None

    @classmethod
    def from_dict(cls, dict: Dict) -> OutputMetadata:
        obj = cls.__new__(cls)
        super(OutputMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj


@dataclass
class OutputWithMetadata:
    """An output with its metadata.
    """

    metadata: OutputMetadata
    output: Output

    @classmethod
    def from_dict(cls, dict: Dict) -> OutputWithMetadata:
        obj = cls.__new__(cls)
        super(OutputWithMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj

    def as_dict(self):
        config = dict()

        config['metadata'] = self.metadata.__dict__
        config['output'] = self.output.as_dict()

        return config
