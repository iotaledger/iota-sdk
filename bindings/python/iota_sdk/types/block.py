# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum
from dataclasses import dataclass
from typing import List, Optional, Union, Dict
from dacite import from_dict
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload
from iota_sdk.utils import Utils


@json
@dataclass
class Block:
    """Represent the object that nodes gossip around the network.

    Attributes:
        protocol_version: The protocol version with which this block was issued.
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        burned_mana: The amount of Mana the Account identified by the IssuerId is at most willing to burn for this block.
        payload: The optional payload of this block.
    """

    protocol_version: int
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    burned_mana: str
    payload: Optional[Union[TaggedDataPayload,
                      TransactionPayload]] = None

    @classmethod
    def from_dict(cls, block_dict: Dict) -> Block:
        """
        The function `from_dict` takes a dictionary that contains the data needed to
        create an instance of the `Block` class.

        Returns:

        An instance of the `Block` class.
        """
        return from_dict(Block, block_dict)

    def id(self) -> HexStr:
        """Rreturns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)


class LedgerInclusionState(str, Enum):
    """Represents whether a block is included in the ledger.

    Attributes:
        noTransaction: The block does not contain a transaction.
        included: The block contains an included transaction.
        conflicting: The block contains a conflicting transaction.
    """
    noTransaction = 'noTransaction'
    included = 'included'
    conflicting = 'conflicting'


class TransactionFailureReason(Enum):
    """Represents the possible reasons for a conflicting transaction.

    Attributes:
        InputUtxoAlreadySpent: The referenced UTXO was already spent.
        ConflictingWithAnotherTx: The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.
        InvalidReferencedUtxo: The referenced UTXO is invalid.
        InvalidTransaction: The transaction is invalid.
        SumInputsOutputsAmountMismatch: The sum of the inputs and output base token amount does not match.
        InvalidUnlockBlockSignature: The unlock block signature is invalid.
        TimelockNotExpired: The configured timelock is not yet expired.
        InvalidNativeTokens: The given native tokens are invalid.
        StorageDepositReturnUnfulfilled: The return amount in a transaction is not fulfilled by the output side.
        InvalidInputUnlock: An input unlock was invalid.
        InvalidInputsCommitment: The inputs commitment is invalid.
        SenderNotUnlocked: The output contains a Sender with an ident (address) which is not unlocked.
        InvalidChainStateTransition: The chain state transition is invalid.
        InvalidTransactionIssuingTime: The referenced input is created after the transaction issuing time.
        InvalidManaAmount: The mana amount is invalid.
        InvalidBlockIssuanceCreditsAmount: The Block Issuance Credits amount is invalid.
        InvalidRewardContextInput: Reward Context Input is invalid.
        InvalidCommitmentContextInput: Commitment Context Input is invalid.
        MissingStakingFeature: Staking Feature is not provided in account output when claiming rewards.
        FailedToClaimStakingReward: Failed to claim staking reward.
        FailedToClaimDelegationReward: Failed to claim delegation reward.
        SemanticValidationFailed: The semantic validation failed for a reason not covered by the previous variants.
    """
    InputUtxoAlreadySpent = 1
    ConflictingWithAnotherTx = 2
    InvalidReferencedUtxo = 3
    InvalidTransaction = 4
    SumInputsOutputsAmountMismatch = 5
    InvalidUnlockBlockSignature = 6
    TimelockNotExpired = 7
    InvalidNativeTokens = 8
    StorageDepositReturnUnfulfilled = 9
    InvalidInputUnlock = 10
    InvalidInputsCommitment = 11
    SenderNotUnlocked = 12
    InvalidChainStateTransition = 13
    InvalidTransactionIssuingTime = 14
    InvalidManaAmount = 15
    InvalidBlockIssuanceCreditsAmount = 16
    InvalidRewardContextInput = 17
    InvalidCommitmentContextInput = 18
    MissingStakingFeature = 19
    FailedToClaimStakingReward = 20
    FailedToClaimDelegationReward = 21
    SemanticValidationFailed = 255

    def __str__(self):
        return {
            1: "The referenced UTXO was already spent.",
            2: "The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.",
            3: "The referenced UTXO is invalid.",
            4: "The transaction is invalid.",
            5: "The sum of the inputs and output base token amount does not match.",
            6: "The unlock block signature is invalid.",
            7: "The configured timelock is not yet expired.",
            8: "The given native tokens are invalid.",
            9: "The return amount in a transaction is not fulfilled by the output side.",
            10: "An input unlock was invalid.",
            11: "The inputs commitment is invalid.",
            12: "The output contains a Sender with an ident (address) which is not unlocked.",
            13: "The chain state transition is invalid.",
            14: "The referenced input is created after the transaction issuing time.",
            15: "The mana amount is invalid.",
            16: "The Block Issuance Credits amount is invalid.",
            17: "Reward Context Input is invalid.",
            18: "Commitment Context Input is invalid.",
            19: "Staking Feature is not provided in account output when claiming rewards.",
            20: "Failed to claim staking reward.",
            21: "Failed to claim delegation reward.",
            255: "The semantic validation failed for a reason not covered by the previous variants."
        }[self.value]


@json
@dataclass
class BlockMetadata:
    """Block Metadata.

    Attributes:
        block_id: The id of the block.
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        is_solid: Whether the block is solid.
        referenced_by_milestone_index: The milestone index referencing the block.
        milestone_index: The milestone index if the block contains a milestone payload.
        ledger_inclusion_state: The ledger inclusion state of the block.
        conflict_reason: The optional conflict reason of the block.
        should_promote: Whether the block should be promoted.
        should_reattach: Whether the block should be reattached.
    """
    block_id: HexStr
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    is_solid: bool
    referenced_by_milestone_index: Optional[int] = None
    milestone_index: Optional[int] = None
    ledger_inclusion_state: Optional[LedgerInclusionState] = None
    conflict_reason: Optional[TransactionFailureReason] = None
    should_promote: Optional[bool] = None
    should_reattach: Optional[bool] = None
