# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.burn import Burn
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import TaggedDataPayload
from enum import Enum
from typing import Optional, List


class RemainderValueStrategyCustomAddress:
    """Custom Address remainder value strategy.
    """
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
    """Remainder value stragegy options.

    Attributes:
        ChangeAddress: Changes the address
        ReuseAddress: Reuses the address
    """
    ChangeAddress = None,
    ReuseAddress = None,

    def as_dict(self):
        return dict({"strategy": self.name, "value": self.value[0]})


class TransactionOptions():
    """Transaction options.

    Attributes:
        remainder_value_strategy (RemainderValueStrategy | RemainderValueStrategyCustomAddress, optional): the remainder value strategy
        tagged_data_payload (TaggedDataPayload, optional): a tagged data payload
        custom_inputs (List[OutputId], optional): an array of custom inputs
        mandatory_inputs (List[OutputId], optional): an array of mandatory inputs
        burn (Burn, optional): some assets to burn in the transaction
        note (str, optional): a note with the transaction
        allow_micro_amount (bool, optional): whether micro amounts should be allowed in this transaction
    """
    def __init__(self, remainder_value_strategy: Optional[RemainderValueStrategy | RemainderValueStrategyCustomAddress] = None,
                 tagged_data_payload: Optional[TaggedDataPayload] = None,
                 custom_inputs: Optional[List[OutputId]] = None,
                 mandatory_inputs: Optional[List[OutputId]] = None,
                 burn: Optional[Burn] = None,
                 note: Optional[str] = None,
                 allow_micro_amount: Optional[bool] = None):
        """Initialize TransactionOptions

        Args:
            remainder_value_strategy (RemainderValueStrategy | RemainderValueStrategyCustomAddress, optional): the remainder value strategy
            tagged_data_payload (TaggedDataPayload, optional): a tagged data payload
            custom_inputs (List[OutputId], optional): an array of custom inputs
            mandatory_inputs (List[OutputId], optional): an array of mandatory inputs
            burn (Burn, optional): some assets to burn in the transaction
            note (str, optional): a note with the transaction
            allow_micro_amount (bool, optional): whether micro amounts should be allowed in this transaction
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
