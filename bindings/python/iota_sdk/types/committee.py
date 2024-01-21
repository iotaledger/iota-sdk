# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import EpochIndex, HexStr, json


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


@json
@dataclass
class Committee:
    """The validator information of the committee.
    Response of GET /api/core/v3/committee

    Attributes:
        committee: The validators of the committee.
        total_stake: The total amount of delegated and staked IOTA coins in the selected committee.
        total_validator_stake: The total amount of staked IOTA coins in the selected committee.
        epoch: The epoch index of the committee.
    """
    committee: List[CommitteeMember]
    total_stake: int = field(metadata=config(
        encoder=str
    ))
    total_validator_stake: int = field(metadata=config(
        encoder=str
    ))
    epoch: EpochIndex


@json
@dataclass
class Validator:
    """Information of a validator.

    Attributes:
        address: Account address of the validator.
        staking_end_epoch: The epoch index until which the validator registered to stake.
        pool_stake: The total stake of the pool, including delegators.
        validator_stake: The stake of a validator.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
        active: Shows whether the validator was active recently.
        latest_supported_protocol_version: The latest protocol version the validator supported.
        latest_supported_protocol_hash: The protocol hash of the latest supported protocol of the validator.
    """
    address: HexStr
    staking_end_epoch: EpochIndex
    pool_stake: int = field(metadata=config(
        encoder=str
    ))
    validator_stake: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))
    active: bool
    latest_supported_protocol_version: int
    latest_supported_protocol_hash: HexStr


@json
@dataclass
class Validators:
    """A paginated list of all registered validators ready for the next epoch and indicates if they were active recently
    (are eligible for committee selection).
    Response of GET /api/core/v3/blocks/validators.

    Attributes:
        stakers: List of registered validators ready for the next epoch.
        page_size: The number of validators returned per one API request with pagination.
        cursor: The cursor that needs to be provided as cursor query parameter to request the next page. If empty, this was the last page.
    """
    stakers: List[Validator]
    page_size: int
    cursor: Optional[str] = None
