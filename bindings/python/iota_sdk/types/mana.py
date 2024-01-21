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

@json
@dataclass
class ManaRewards:
    """The mana rewards of an account or delegation output.
    Response of GET /api/core/v3/rewards/{outputId}.

    Attributes:
        start_epoch: First epoch for which rewards can be claimed. This value is useful for checking if rewards have expired (by comparing against the staking or delegation start) or would expire soon (by checking its relation to the rewards retention period).
        end_epoch: Last epoch for which rewards can be claimed.
        rewards: Amount of totally available decayed rewards the requested output may claim.
        latest_committed_epoch_pool_rewards: Rewards of the latest committed epoch of the staking pool to which this validator or delegator belongs. The ratio of this value and the maximally possible rewards for the latest committed epoch can be used to determine how well the validator of this staking pool performed in that epoch. Note that if the pool was not part of the committee in the latest committed epoch, this value is 0.
    """
    start_epoch: int
    end_epoch: int
    rewards: int = field(metadata=config(
        encoder=str
    ))
    latest_committed_epoch_pool_rewards: int = field(metadata=config(
        encoder=str
    ))
