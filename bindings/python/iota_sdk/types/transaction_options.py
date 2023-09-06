# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum
from typing import Optional, List, Dict
from dataclasses import dataclass
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import json
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import TaggedDataPayload


@json
@dataclass
class RemainderValueStrategyCustomAddress:
    """Remainder value strategy for custom addresses.

    Attributes:
        address: An address to move the remainder value to.
        key_index: The address key index.
        internal: Determines if an address is a public or an internal (change) address.
        used: Indicates whether an address has been used already.
    """

    address: str
    key_index: int
    internal: bool
    used: bool

    @staticmethod
    def _to_dict_custom(config: Dict[str, any]) -> Dict[str, any]:
        return dict({"strategy": "CustomAddress", "value": config})


class RemainderValueStrategy(Enum):
    """Remainder value stragegy variants.

    Attributes:
        ChangeAddress: Allows to move the remainder value to a change address.
        ReuseAddress: Allows to keep the remainder value on the source address.
    """
    ChangeAddress = None,
    ReuseAddress = None,

    def to_dict(self):
        return dict({"strategy": self.name, "value": self.value[0]})


@json
@dataclass
class TransactionOptions():
    """Transaction options.

    Attributes:
        remainder_value_strategy: The strategy applied for base coin remainders.
        tagged_data_payload: An optional tagged data payload.
        custom_inputs: If custom inputs are provided only those are used. If also other additional inputs should be used, `mandatory_inputs` should be used instead.
        mandatory_inputs: Inputs that must be used for the transaction.
        burn: Specifies what needs to be burned during input selection.
        note: A string attached to the transaction.
        allow_micro_amount: Whether to allow sending a micro amount.
    """

    remainder_value_strategy: Optional[RemainderValueStrategy |
                                       RemainderValueStrategyCustomAddress] = None
    tagged_data_payload: Optional[TaggedDataPayload] = None
    custom_inputs: Optional[List[OutputId]] = None
    mandatory_inputs: Optional[List[OutputId]] = None
    burn: Optional[Burn] = None
    note: Optional[str] = None
    allow_micro_amount: Optional[bool] = None
