# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Dict, Optional, List, Union
from dacite import from_dict
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.feature import SenderFeature, IssuerFeature, MetadataFeature, TagFeature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition, ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAccountAddressUnlockCondition


class OutputType(IntEnum):
    """Output types.

    Attributes:
        Basic (3): A basic output.
        Account (4): An account output.
        Foundry (5): A foundry output.
        Nft (6): An NFT output.
    """
    Basic = 3
    Account = 4
    Foundry = 5
    Nft = 6


@json
@dataclass
class Output():
    """An output in a UTXO ledger.
    """
    type: int


@json
@dataclass
class BasicOutput(Output):
    """Describes a basic output.
    Attributes:
        amount :
            The base coin amount of the output.
        mana :
            Amount of stored Mana held by this output.
        unlock_conditions :
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        native_tokens :
            Native tokens added to the new output.
        type :
            The type of output.
    """
    amount: str
    mana: str
    unlockConditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                           TimelockUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = None
    nativeTokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Basic),
        init=False)


@json
@dataclass
class AccountOutput(Output):
    """Describes an account output.
    Attributes:
        amount :
            The base coin amount of the output.
        mana :
            Amount of stored Mana held by this output.
        unlock_conditions:
            The conditions to unlock the output.
        account_id :
            The account ID if it's an account output.
        state_index :
            A counter that must increase by 1 every time the account is state transitioned.
        state_metadata :
            Metadata that can only be changed by the state controller.
        foundry_counter :
            A counter that denotes the number of foundries created by this account output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        native_tokens :
            Native tokens added to the new output.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    amount: str
    mana: str
    account_id: HexStr
    stateIndex: int
    foundry_counter: int
    unlock_conditions: List[Union[StateControllerAddressUnlockCondition,
                                  GovernorAddressUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature]]] = None
    immutable_features: Optional[List[Union[IssuerFeature,
                                            MetadataFeature]]] = None
    state_metadata: Optional[HexStr] = None
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Account),
        init=False)


@json
@dataclass
class FoundryOutput(Output):
    """Describes a foundry output.
    Attributes:
        amount :
            The base coin amount of the output.
        unlock_conditions :
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        native_tokens :
            Native tokens added to the new output.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        serial_number :
            The serial number of the foundry with respect to the controlling account.
        token_scheme :
            Defines the supply control scheme of the tokens controlled by the foundry. Currently only a simple scheme is supported.
        type :
            The type of output.
    """
    amount: str
    serial_number: int
    token_scheme: SimpleTokenScheme
    unlock_conditions: List[ImmutableAccountAddressUnlockCondition]
    features: Optional[List[MetadataFeature]] = None
    immutable_features: Optional[List[MetadataFeature]] = None
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Foundry),
        init=False)


@json
@dataclass
class NftOutput(Output):
    """Describes an NFT output.
    Attributes:
        amount :
            The base coin amount of the output.
        mana :
            Amount of stored Mana held by this output.
        unlock_conditions :
            The conditions to unlock the output.
        nft_id :
            The NFT ID if it's an NFT output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        native_tokens :
            Native tokens added to the new output.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    amount: str
    mana: str
    nft_id: HexStr
    unlock_conditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition,
                                  StorageDepositReturnUnlockCondition, TimelockUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = None
    immutable_features: Optional[List[Union[
        IssuerFeature, MetadataFeature]]] = None
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(default_factory=lambda: int(OutputType.Nft), init=False)


@json
@dataclass
class OutputMetadata:
    """Metadata about an output.

    Attributes:
        block_id: The ID of the block in which the output appeared in.
        transaction_id: The ID of the transaction in which the output was created.
        output_index: The index of the output within the corresponding transaction.
        is_spent: Whether the output is already spent.
        milestone_index_booked: The index of the milestone which booked/created the output.
        milestone_timestamp_booked: The timestamp of the milestone which booked/created the output.
        ledger_index: The current ledger index.
        milestone_index_spent: The index of the milestone which spent the output.
        milestone_timestamp_spent: The timestamp of the milestone which spent the output.
        transaction_id_spent: The ID of the transaction that spent the output.
    """
    block_id: HexStr
    transaction_id: HexStr
    output_index: int
    is_spent: bool
    milestone_index_booked: int
    milestone_timestamp_booked: int
    ledger_index: int
    milestone_index_spent: Optional[int] = None
    milestone_timestamp_spent: Optional[int] = None
    transaction_id_spent: Optional[HexStr] = None


@json
@dataclass
class OutputWithMetadata:
    """An output with its metadata.

    Attributes:
        metadata: The `OutputMetadata` object that belongs to `output`.
        output: An `Output` object.
    """

    metadata: OutputMetadata
    output: Union[AccountOutput, FoundryOutput, NftOutput, BasicOutput]

    @classmethod
    def from_dict(cls, dict: Dict) -> OutputWithMetadata:
        obj = cls.__new__(cls)
        super(OutputWithMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj

    def as_dict(self):
        config = dict()

        config['metadata'] = self.metadata.__dict__
        config['output'] = self.output.as_dict()

        return config


def output_from_dict(
        output: Dict[str, any]) -> Union[BasicOutput, AccountOutput, FoundryOutput, NftOutput, Output]:
    output_type = OutputType(output['type'])

    if output_type == OutputType.Basic:
        return BasicOutput.from_dict(output)
    if output_type == OutputType.Account:
        return AccountOutput.from_dict(output)
    if output_type == OutputType.Foundry:
        return FoundryOutput.from_dict(output)
    if output_type == OutputType.Nft:
        return NftOutput.from_dict(output)

    return Output.from_dict(output)
