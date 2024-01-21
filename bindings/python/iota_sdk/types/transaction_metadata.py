# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum
from typing import Optional
from dataclasses import dataclass
from iota_sdk.types.common import HexStr, json


class TransactionState(Enum):
    """Describes the state of a transaction.

    Attributes:
        Pending: Not included yet.
        Accepted: Included.
        Confirmed: Included and its included block is confirmed.
        Finalized: Included, its included block is finalized and cannot be reverted anymore.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 0
    Accepted = 1
    Confirmed = 2
    Finalized = 3
    Failed = 4


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
        TransactionCapabilityNativeTokenBurningNotAllowed: Burning of native tokens is not allowed in the transaction capabilities.
        TransactionCapabilityManaBurningNotAllowed: Burning of mana is not allowed in the transaction capabilities.
        TransactionCapabilityAccountDestructionNotAllowed: Destruction of accounts is not allowed in the transaction capabilities.
        TransactionCapabilityAnchorDestructionNotAllowed: Destruction of anchors is not allowed in the transaction capabilities.
        TransactionCapabilityFoundryDestructionNotAllowed: Destruction of foundries is not allowed in the transaction capabilities.
        TransactionCapabilityNftDestructionNotAllowed: Destruction of nfts is not allowed in the transaction capabilities.
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
    SenderNotUnlocked = 11
    InvalidChainStateTransition = 12
    InvalidTransactionIssuingTime = 13
    InvalidManaAmount = 14
    InvalidBlockIssuanceCreditsAmount = 15
    InvalidRewardContextInput = 16
    InvalidCommitmentContextInput = 17
    MissingStakingFeature = 18
    FailedToClaimStakingReward = 19
    FailedToClaimDelegationReward = 20
    TransactionCapabilityNativeTokenBurningNotAllowed = 21
    TransactionCapabilityManaBurningNotAllowed = 22
    TransactionCapabilityAccountDestructionNotAllowed = 23
    TransactionCapabilityAnchorDestructionNotAllowed = 24
    TransactionCapabilityFoundryDestructionNotAllowed = 25
    TransactionCapabilityNftDestructionNotAllowed = 26
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
            11: "The output contains a Sender with an ident (address) which is not unlocked.",
            12: "The chain state transition is invalid.",
            13: "The referenced input is created after the transaction issuing time.",
            14: "The mana amount is invalid.",
            15: "The Block Issuance Credits amount is invalid.",
            16: "Reward Context Input is invalid.",
            17: "Commitment Context Input is invalid.",
            18: "Staking Feature is not provided in account output when claiming rewards.",
            19: "Failed to claim staking reward.",
            20: "Failed to claim delegation reward.",
            21: "Burning of native tokens is not allowed in the transaction capabilities.",
            22: "Burning of mana is not allowed in the transaction capabilities.",
            23: "Destruction of accounts is not allowed in the transaction capabilities.",
            24: "Destruction of anchors is not allowed in the transaction capabilities.",
            25: "Destruction of foundries is not allowed in the transaction capabilities.",
            26: "Destruction of nfts is not allowed in the transaction capabilities.",
            255: "The semantic validation failed for a reason not covered by the previous variants."
        }[self.value]


@json
@dataclass
class TransactionMetadata:
    """Response of a GET transaction metadata REST API call.

    Attributes:
        transaction_id: TODO
        transaction_state: TODO
        transaction_failure_reason: TODO
    """
    transaction_id: HexStr
    transaction_state: TransactionState
    transaction_failure_reason: Optional[TransactionFailureReason] = None
