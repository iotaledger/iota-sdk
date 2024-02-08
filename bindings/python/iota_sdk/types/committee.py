# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class CommitteeMember:
    """Information of a committee member.

    Attributes:
        address: Account address of the validator.
        pool_stake: The total stake of the pool, including delegators.
        validator_stake: The stake of a validator.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
    """
    address: HexStr
    pool_stake: int = field(metadata=config(
        encoder=str
    ))
    validator_stake: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))
