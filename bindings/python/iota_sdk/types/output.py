# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, Optional
from iota_sdk.types.common import HexStr

# TODO: https://github.com/iotaledger/iota-sdk/issues/129
# @dataclass
# class Output():
#     def __init__(self, type, sender=None, issuer=None, data=None, tag=None):
#         """Initialize an output

#         Parameters
#         ----------
#         type : OutputType
#             The type of output
#         """
#         self.type = type


#     @classmethod
#     def from_dict(cls, output_dict: Dict) -> Output:
#         obj = cls.__new__(cls)
#         super(Output, obj).__init__()
#         for k, v in output_dict.items():
#             setattr(obj, k, v)
#         return obj

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
    output: Any  # TODO: Output https://github.com/iotaledger/iota-sdk/issues/129

    @classmethod
    def from_dict(cls, dict: Dict) -> OutputWithMetadata:
        obj = cls.__new__(cls)
        super(OutputWithMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj
