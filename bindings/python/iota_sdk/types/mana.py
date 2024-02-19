# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class ManaAllotment:
    """An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
    in the form of Block Issuance Credits to the account.

    Attributes:
        account_id: The unique identifier of the account.
        mana: The alloted amount of mana.
    """
    account_id: HexStr
    mana: int = field(metadata=config(
        encoder=str
    ))
