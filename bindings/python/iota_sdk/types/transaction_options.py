# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.burn import Burn
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import TaggedDataPayload
from enum import Enum
from typing import Optional, List


class RemainderValueStrategyCustomAddress:
    def __init__(self,
                 address: str,
                 key_index: int,
                 internal: bool,
                 used: bool):
        self.address = address
        self.keyIndex = key_index
        self.internal = internal
        self.used = used

    def as_dict(self):
        return dict({"strategy": "CustomAddress", "value": self.__dict__})


class RemainderValueStrategy(Enum):
    ChangeAddress = None,
    ReuseAddress = None,

    def as_dict(self):
        return dict({"strategy": self.name, "value": self.value[0]})


class TransactionOptions():
    def __init__(self, remainder_value_strategy: Optional[RemainderValueStrategy | RemainderValueStrategyCustomAddress] = None,
                 tagged_data_payload: Optional[TaggedDataPayload] = None,
                 custom_inputs: Optional[List[OutputId]] = None,
                 mandatory_inputs: Optional[List[OutputId]] = None,
                 burn: Optional[Burn] = None,
                 note: Optional[str] = None,
                 allow_micro_amount: Optional[bool] = None):
        """Initialize TransactionOptions
        """
        self.remainder_value_strategy = remainder_value_strategy
        self.tagged_data_payload = tagged_data_payload
        self.custom_inputs = custom_inputs
        self.mandatory_inputs = mandatory_inputs
        self.burn = burn
        self.note = note
        self.allow_micro_amount = allow_micro_amount

    def as_dict(self):
        return dict(self.__dict__)
