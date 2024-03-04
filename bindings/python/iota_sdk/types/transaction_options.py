# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum
from typing import Optional, List, Union
from dataclasses import dataclass, field
from iota_sdk.types.address import Address
from iota_sdk.types.burn import Burn
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import TaggedDataPayload


@json
@dataclass
class RemainderValueStrategyCustomAddress:
    """Remainder value strategy for custom addresses.

    Attributes:
        value: An address to move the remainder value to.
    """
    strategy: str = field(default_factory=lambda: 'CustomAddress', init=False)
    value: Address


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
        }


@json
@dataclass
class TransactionOptions:
    """Transaction options.

    Attributes:
        remainder_value_strategy: The strategy applied for base coin remainders.
        tagged_data_payload: An optional tagged data payload.
        required_inputs: Inputs that must be used for the transaction.
        burn: Specifies what needs to be burned in the transaction.
        note: A string attached to the transaction.
        allow_micro_amount: Whether to allow sending a micro amount.
        allow_additional_input_selection: Whether to allow the selection of additional inputs for this transaction.
        mana_allotments: Mana allotments for the transaction.
        issuer_id: Optional block issuer to which the transaction will have required mana allotted.
    """
    remainder_value_strategy: Optional[Union[RemainderValueStrategy,
                                             RemainderValueStrategyCustomAddress]] = None
    tagged_data_payload: Optional[TaggedDataPayload] = None
    required_inputs: Optional[List[OutputId]] = None
    burn: Optional[Burn] = None
    note: Optional[str] = None
    allow_micro_amount: Optional[bool] = None
    allow_additional_input_selection: Optional[bool] = None
    mana_allotments: Optional[dict[HexStr, int]] = None
    issuer_id: Optional[HexStr] = None
