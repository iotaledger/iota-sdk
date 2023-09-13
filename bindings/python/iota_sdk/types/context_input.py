# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from iota_sdk.types.common import HexStr, json


class ContextInputType(IntEnum):
    """Context input types.
    """
    Commitment = 0
    BlockIssuanceCredit = 1
    Reward = 2


@json
@dataclass
class ContextInput():
    """Base class for context inputs.
    """
    type: int


@json
@dataclass
class CommitmentContextInput(ContextInput):
    """A Commitment Input allows referencing a commitment to a certain slot.

    Attributes:
        type: The type of commitment input.
        commitment_id: The commitment identifier to reference to.
    """
    # TODO Replace with a proper SlotIndex type
    commitment_id: HexStr
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Commitment),
        init=False)


@json
@dataclass
class BlockIssuanceCreditContextInput(ContextInput):
    """A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
    the BIC vector of a specific slot.

    Attributes:
        type: The type of commitment input.
        account_id: The BIC of an account to use.
    """
    account_id: HexStr
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.BlockIssuanceCredit),
        init=False)


@json
@dataclass
class RewardContextInput(ContextInput):
    """A Reward Context Input indicates which transaction Input is claiming Mana rewards.

    Attributes:
        type: The type of commitment input.
        index: The index of the transaction input for which to claim rewards.
    """
    index: int
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Reward),
        init=False)
