# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum
from typing import Optional, List, Union
from dataclasses import dataclass
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.context_input import ContextInput
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

    def to_dict(self) -> dict:
        """Custom dict conversion.
        """

        return {
            'strategy': 'CustomAddress',
            'value': {
                'address': self.address,
                'keyIndex': self.key_index,
                'internal': self.internal,
                'used': self.used
            }
        }


class RemainderValueStrategy(Enum):
    """Remainder value strategy variants.

    Attributes:
        ReuseAddress: Allows to keep the remainder value on the source address.
    """
    ReuseAddress = None

    def to_dict(self) -> dict:
        """Custom dict conversion.
        """

        return {
            'strategy': self.name,
            'value': self.value[0]
        }


@json
@dataclass
class TransactionOptions:
    """Transaction options.

    Attributes:
        remainder_value_strategy: The strategy applied for base coin remainders.
        tagged_data_payload: An optional tagged data payload.
        context_inputs: Transaction context inputs to include.
        required_inputs: Inputs that must be used for the transaction.
        burn: Specifies what needs to be burned during input selection.
        note: A string attached to the transaction.
        allow_micro_amount: Whether to allow sending a micro amount.
        allow_additional_input_selection: Whether to allow the selection of additional inputs for this transaction.
        capabilities: Transaction capabilities.
        mana_allotments: Mana allotments for the transaction.
        issuer_id: Optional block issuer to which the transaction will have required mana allotted.
    """

    def __init__(self, remainder_value_strategy: Optional[Union[RemainderValueStrategy, RemainderValueStrategyCustomAddress]] = None,
                 tagged_data_payload: Optional[TaggedDataPayload] = None,
                 context_inputs: Optional[List[ContextInput]] = None,
                 required_inputs: Optional[List[OutputId]] = None,
                 burn: Optional[Burn] = None,
                 note: Optional[str] = None,
                 allow_micro_amount: Optional[bool] = None,
                 allow_additional_input_selection: Optional[bool] = None,
                 capabilities: Optional[HexStr] = None,
                 mana_allotments: Optional[dict[HexStr, int]] = None,
                 issuer_id: Optional[HexStr] = None):
        """Initialize transaction options.
        """
        self.remainder_value_strategy = remainder_value_strategy
        self.tagged_data_payload = tagged_data_payload
        self.context_inputs = context_inputs
        self.required_inputs = required_inputs
        self.burn = burn
        self.note = note
        self.allow_micro_amount = allow_micro_amount
        self.allow_additional_input_selection = allow_additional_input_selection
        self.capabilities = capabilities
        self.mana_allotments = mana_allotments
        self.issuer_id = issuer_id
