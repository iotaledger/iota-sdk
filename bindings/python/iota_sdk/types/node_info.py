# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional
from dataclasses_json import config
from iota_sdk.types.common import EpochIndex, json, SlotIndex
from iota_sdk.types.slot import SlotCommitmentId


@json
@dataclass
class StatusResponse:
    """Node status.

    Attributes:
        is_healthy: Tells whether the node is healthy or not.
        is_network_healthy: Tells whether the network is healthy (finalization is not delayed).
        accepted_tangle_time: A notion of time that is anchored to the latest accepted block.
        relative_accepted_tangle_time: The time after Accepted Tangle Time has advanced with the system clock.
        confirmed_tangle_time: A notion of time that is anchored to the latest confirmed block.
        relative_confirmed_tangle_time: The time after Confirmed Tangle Time has advanced with the system clock.
        latest_commitment_id: The latest slot that the node has committed to.
        latest_finalized_slot: The index of the latest finalized slot.
        latest_accepted_block_slot: The slot index of the latest accepted block.
        latest_confirmed_block_slot: The slot index of the latest confirmed block.
        pruning_epoch: The index of the epoch before which the tangle history is pruned.
    """
    is_healthy: bool
    is_network_healthy: bool
    accepted_tangle_time: int = field(metadata=config(
        encoder=str
    ))
    relative_accepted_tangle_time: int = field(metadata=config(
        encoder=str
    ))
    confirmed_tangle_time: int = field(metadata=config(
        encoder=str
    ))
    relative_confirmed_tangle_time: int = field(metadata=config(
        encoder=str
    ))
    latest_commitment_id: SlotCommitmentId
    latest_finalized_slot: SlotIndex
    latest_accepted_block_slot: SlotIndex
    latest_confirmed_block_slot: SlotIndex
    pruning_epoch: EpochIndex


@json
@dataclass
class StorageScoreParameters:
    """Defines the parameters of storage score calculations on objects which take node resources.

    Attributes:
        storage_cost: Defines the number of IOTA tokens required per unit of storage score.
        factor_data: Defines the factor to be used for data only fields.
        offset_output_overhead: Defines the offset to be applied to all outputs for the overhead of handling them in storage.
        offset_ed25519_block_issuer_key: Defines the offset to be used for block issuer feature public keys.
        offset_staking_feature: Defines the offset to be used for staking feature.
        offset_delegation: Defines the offset to be used for delegation output.
    """
    storage_cost: int = field(metadata=config(
        encoder=str
    ))
    factor_data: int
    offset_output_overhead: int = field(metadata=config(
        encoder=str
    ))
    offset_ed25519_block_issuer_key: int = field(metadata=config(
        encoder=str
    ))
    offset_staking_feature: int = field(metadata=config(
        encoder=str
    ))
    offset_delegation: int = field(metadata=config(
        encoder=str
    ))

    def as_dict(self):
        """Converts this object to a dict.
        """
        res = {k: v for k, v in self.__dict__.items() if v is not None}
        return res


@json
@dataclass
class WorkScoreParameters:
    """Work Score Parameters lists the work score of each type, it is used to denote the computation costs of processing an object.

    Attributes:
        data_byte: Data_kibibyte accounts for the network traffic per kibibyte.
        block: Block accounts for work done to process a block in the node software.
        input: Input accounts for loading the UTXO from the database and performing the mana calculations.
        context_input: Context_input accounts for loading and checking the context input.
        output: Output accounts for storing the UTXO in the database.
        native_token: Native_token accounts for calculations done with native tokens.
        staking: Staking accounts for the existence of a staking feature in the output.
        block_issuer: BlockIssuer accounts for the existence of a block issuer feature in the output.
        allotment: Allotment accounts for accessing the account-based ledger to transform the mana to block issuance credits.
        signature_ed25519: SignatureEd25519 accounts for an Ed25519 signature check.
    """
    data_byte: int
    block: int
    input: int
    context_input: int
    output: int
    native_token: int
    staking: int
    block_issuer: int
    allotment: int
    signature_ed25519: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        res = {k: v for k, v in self.__dict__.items() if v is not None}
        if res["rentStructure"]:
            res["rentStructure"] = res["rentStructure"].as_dict()
        return res


@json
@dataclass
class CongestionControlParameters:
    """Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).

    Attributes:
        min_reference_mana_cost: The minimum value of the reference Mana cost.
        increase: The increase step size of the reference Mana cost.
        decrease: The decrease step size of the reference Mana cost.
        increase_threshold: The threshold for increasing the reference Mana cost.
        decrease_threshold: The threshold for decreasing the reference Mana cost.
        scheduler_rate: The rate at which the scheduler runs in workscore units per second.
        max_buffer_size: The maximum size of the buffer in the scheduler.
        max_validation_buffer_size: The maximum number of blocks in the validation buffer.
    """
    min_reference_mana_cost: int = field(metadata=config(
        encoder=str
    ))
    increase: int = field(metadata=config(
        encoder=str
    ))
    decrease: int = field(metadata=config(
        encoder=str
    ))
    increase_threshold: int
    decrease_threshold: int
    scheduler_rate: int
    max_buffer_size: int
    max_validation_buffer_size: int


@json
@dataclass
class VersionSignalingParameters:
    """Version Signaling defines the parameters used by signaling protocol parameters upgrade.

    Attributes:
        window_size: The size of the window in epochs to find which version of protocol parameters was most signaled, from current_epoch - window_size to current_epoch.
        window_target_ratio: The target number of supporters for a version to win in a window_size.
        activation_offset: The offset in epochs to activate the new version of protocol parameters.
    """
    window_size: int
    window_target_ratio: int
    activation_offset: int


@json
@dataclass
class ManaParameters:
    """ManaParameters defines the parameters used by mana calculation.

    Attributes:
        bits_count: The number of bits used to represent Mana.
        generation_rate: The amount of potential Mana generated by 1 IOTA in 1 slot.
        generation_rate_exponent: The scaling of generation_rate expressed as an exponent of 2.
        decay_factors: A lookup table of epoch index diff to mana decay factor (slice index 0 = 1 epoch).
        decay_factors_exponent: The scaling of decay_factors expressed as an exponent of 2.
        decay_factor_epochs_sum: An integer approximation of the sum of decay over epochs.
        decay_factor_epochs_sum_exponent: The scaling of decay_factor_epochs_sum expressed as an exponent of 2.
        annual_decay_factor_percentage: Decay factor for 1 year.
    """
    bits_count: int
    generation_rate: int
    generation_rate_exponent: int
    decay_factors: List[int]
    decay_factors_exponent: int
    decay_factor_epochs_sum: int
    decay_factor_epochs_sum_exponent: int
    annual_decay_factor_percentage: int


@json
@dataclass
class RewardsParameters:
    """Rewards Parameters defines the parameters that are used to calculate Mana rewards.

    Attributes:
        profit_margin_exponent: Used for shift operation during calculation of profit margin.
        bootstrapping_duration: The length of the bootstrapping phase in epochs.
        reward_to_generation_ratio: The ratio of the final rewards rate to the generation rate of Mana.
        initial_target_rewards_rate: The rate of Mana rewards at the start of the bootstrapping phase.
        final_target_rewards_rate: The rate of Mana rewards after the bootstrapping phase.
        pool_coefficient_exponent: The exponent used for shifting operation during the pool rewards calculations.
        retention_period: The number of epochs for which rewards are retained.
    """
    profit_margin_exponent: int
    bootstrapping_duration: int
    reward_to_generation_ratio: int
    initial_target_rewards_rate: int = field(metadata=config(
        encoder=str
    ))
    final_target_rewards_rate: int = field(metadata=config(
        encoder=str
    ))
    pool_coefficient_exponent: int
    retention_period: int


@json
@dataclass
class ProtocolParameters:
    """The protocol parameters.

    Attributes:
        type: Set to value 0 to denote a IOTA 2.0 protocol parameter.
        version: Protocol version used by the network.
        network_name: The Name of the network from which the networkId is derived.
        bech32_hrp: Tells whether the node supports mainnet or testnet addresses.
                    Value `iota` indicates that the node supports mainnet addresses.
                    Value `atoi` indicates that the node supports testnet addresses.
        storage_score_parameters: The storage score parameters used by given node/network.
        work_score_parameters: Work Score Parameters lists the work score of each type, it is used to denote the computation costs of processing an object.
        token_supply: Current supply of the base token. Plain string encoded number.
        genesis_slot: Defines the slot of the genesis.
        genesis_unix_timestamp: The genesis timestamp at which the slots start to count.
        slot_duration_in_seconds: The duration of a slot, in seconds.
        slots_per_epoch_exponent: The number of slots in an epoch expressed as an exponent of 2.
        mana_parameters: ManaParameters defines the parameters used by mana calculation.
        staking_unbonding_period: The unbonding period in epochs before an account can stop staking.
        validation_blocks_per_slot: Validation Blocks Per Slot is the number of validation blocks that each validator should issue each slot.
        punishment_epochs: The number of epochs worth of Mana that a node is punished with for each additional validation block it issues.
        liveness_threshold_lower_bound: Used by tip-selection to determine if a block is eligible by evaluating issuing times.
        liveness_threshold_upper_bound: Used by tip-selection to determine if a block is eligible by evaluating issuing times.
        min_committable_age: Min_committable_age is the minimum age relative to the accepted tangle time slot index that a slot can be committed.
        max_committable_age: Max_committable_age is the maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing time.
        epoch_nearing_threshold: Determine the slot that should trigger a new committee selection for the next and upcoming epoch.
        congestion_control_parameters: Congestion Control Parameters defines the parameters used to calculate the Reference Mana Cost (RMC).
        version_signaling_parameters: The version signaling parameters.
        rewards_parameters: Rewards Parameters defines the parameters that are used to calculate Mana rewards.
        target_committee_size: Defines the target size of the committee. If there's fewer candidates the actual committee size could be smaller in a given epoch.
        chain_switching_threshold: Defines the number of heavier slots that a chain needs to be ahead of the current chain to be considered for switching.
    """
    type: int
    version: int
    network_name: str
    bech32_hrp: str
    storage_score_parameters: StorageScoreParameters
    work_score_parameters: WorkScoreParameters
    mana_parameters: ManaParameters
    token_supply: int = field(metadata=config(
        encoder=str
    ))
    genesis_slot: int
    genesis_unix_timestamp: int = field(metadata=config(
        encoder=str
    ))
    slot_duration_in_seconds: int
    slots_per_epoch_exponent: int
    staking_unbonding_period: int
    validation_blocks_per_slot: int
    punishment_epochs: int
    liveness_threshold_lower_bound: int
    liveness_threshold_upper_bound: int
    min_committable_age: int
    max_committable_age: int
    epoch_nearing_threshold: int
    congestion_control_parameters: CongestionControlParameters
    version_signaling_parameters: VersionSignalingParameters
    rewards_parameters: RewardsParameters
    target_committee_size: int
    chain_switching_threshold: int


@json
@dataclass
class BaseTokenResponse:
    """The base coin info.

    Attributes:
        name: The name of the base token of the network.
        ticker_symbol: Ticker symbol of the token to be displayed on trading platforms.
        unit: The primary unit of the token.
        subunit: The name of the smallest possible denomination of the primary unit. subunit * 10^decimals = unit.
        decimals: Number of decimals the primary unit is divisible up to.
    """
    name: str
    ticker_symbol: str
    unit: str
    decimals: int
    subunit: Optional[str] = None
