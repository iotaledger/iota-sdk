# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum
from typing import Optional, List, Union
from iota_sdk.types.burn import Burn
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import TaggedDataPayload


class RemainderValueStrategyCustomAddress:
    """Remainder value strategy for custom addresses.

    Attributes:
        address: An address to move the remainder value to.
        key_index: The address key index.
        internal: Determines if an address is a public or an internal (change) address.
        used: Indicates whether an address has been used already.
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
        """Converts this object to a dict.
        """
        return dict({"strategy": "CustomAddress", "value": self.__dict__})


class RemainderValueStrategy(Enum):
    """Remainder value strategy variants.

    Attributes:
        ChangeAddress: Allows to move the remainder value to a change address.
        ReuseAddress: Allows to keep the remainder value on the source address.
    """
    ChangeAddress = None
    ReuseAddress = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        return dict({"strategy": self.name, "value": self.value[0]})


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

    def __init__(self, remainder_value_strategy: Optional[Union[RemainderValueStrategy, RemainderValueStrategyCustomAddress]] = None,
                 tagged_data_payload: Optional[TaggedDataPayload] = None,
                 custom_inputs: Optional[List[OutputId]] = None,
                 mandatory_inputs: Optional[List[OutputId]] = None,
                 burn: Optional[Burn] = None,
                 note: Optional[str] = None,
                 allow_micro_amount: Optional[bool] = None):
        """Initialize transaction options.
        """
        self.remainder_value_strategy = remainder_value_strategy
        self.tagged_data_payload = tagged_data_payload
        self.custom_inputs = custom_inputs
        self.mandatory_inputs = mandatory_inputs
        self.burn = burn
        self.note = note
        self.allow_micro_amount = allow_micro_amount

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'remainder_value_strategy' in config:
            config['remainder_value_strategy'] = config['remainder_value_strategy'].as_dict()

        return config
