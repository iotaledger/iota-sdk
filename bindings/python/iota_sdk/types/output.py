# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Dict, Optional, List, TypeAlias, Union, Any
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json, EpochIndex
from iota_sdk.types.feature import deserialize_features, SenderFeature, IssuerFeature, MetadataFeature, TagFeature, NativeTokenFeature
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import deserialize_unlock_conditions, AddressUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition, ExpirationUnlockCondition, ImmutableAccountAddressUnlockCondition
from iota_sdk.types.address import AccountAddress


class OutputType(IntEnum):
    """Output types.

    Attributes:
        Basic (0): A basic output.
        Account (1): An account output.
        Anchor (2): An anchor output.
        Foundry (3): A foundry output.
        Nft (4): An NFT output.
        Delegation (5): A delegation output.

    """
    Basic = 0
    Account = 1
    Anchor = 2
    Foundry = 3
    Nft = 4
    Delegation = 5


@json
@dataclass
class BasicOutput:
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
        type :
            The type of output.
    """
    type: int = field(
        default_factory=lambda: int(
            OutputType.Basic),
        init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    mana: int = field(metadata=config(
        encoder=str
    ))
    unlock_conditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                                  TimelockUnlockCondition]] = field(metadata=config(
                                                                    decoder=deserialize_unlock_conditions
                                                                    ))
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature, NativeTokenFeature]]] = field(default=None,
                                                                                       metadata=config(
                                                                                           decoder=deserialize_features
                                                                                       ))


@json
@dataclass
class AccountOutput:
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
        foundry_counter :
            A counter that denotes the number of foundries created by this account output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    type: int = field(
        default_factory=lambda: int(
            OutputType.Account),
        init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    mana: int = field(metadata=config(
        encoder=str
    ))
    account_id: HexStr
    foundry_counter: int
    unlock_conditions: List[AddressUnlockCondition] = field(
        metadata=config(
            decoder=deserialize_unlock_conditions
        ))
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature]]] = field(default=None,
                                                       metadata=config(
                                                           decoder=deserialize_features
                                                       ))
    immutable_features: Optional[List[Union[IssuerFeature,
                                            MetadataFeature]]] = field(default=None,
                                                                       metadata=config(
                                                                           decoder=deserialize_features
                                                                       ))


@json
@dataclass
class AnchorOutput:
    """Describes an anchor output.

    Attributes:
        amount :
            The base coin amount of the output.
        mana :
            Amount of stored Mana held by this output.
        anchor_id :
            The anchor ID if it's an anchor output.
        state_index :
            A counter that must increase by 1 every time the anchor is state transitioned.
        unlock_conditions:
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    type: int = field(
        default_factory=lambda: int(
            OutputType.Anchor),
        init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    mana: int = field(metadata=config(
        encoder=str
    ))
    anchor_id: HexStr
    state_index: int
    unlock_conditions: List[Union[StateControllerAddressUnlockCondition,
                                  GovernorAddressUnlockCondition]] = field(
        metadata=config(
            decoder=deserialize_unlock_conditions
        ))
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature]]] = field(default=None,
                                                       metadata=config(
                                                           decoder=deserialize_features
                                                       ))
    immutable_features: Optional[List[Union[IssuerFeature,
                                            MetadataFeature]]] = field(default=None,
                                                                       metadata=config(
                                                                           decoder=deserialize_features
                                                                       ))


@json
@dataclass
class FoundryOutput:
    """Describes a foundry output.

    Attributes:
        amount :
            The base coin amount of the output.
        unlock_conditions :
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        serial_number :
            The serial number of the foundry with respect to the controlling account.
        token_scheme :
            Defines the supply control scheme of the tokens controlled by the foundry. Currently only a simple scheme is supported.
        type :
            The type of output.
    """
    type: int = field(
        default_factory=lambda: int(
            OutputType.Foundry),
        init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    serial_number: int
    token_scheme: SimpleTokenScheme
    unlock_conditions: List[ImmutableAccountAddressUnlockCondition]
    features: Optional[List[Union[MetadataFeature, NativeTokenFeature]]] = field(default=None,
                                                                                 metadata=config(
                                                                                     decoder=deserialize_features
                                                                                 ))
    immutable_features: Optional[List[MetadataFeature]] = field(default=None,
                                                                metadata=config(
                                                                    decoder=deserialize_features
                                                                ))


@json
@dataclass
class NftOutput:
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
        immutable_features :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    type: int = field(default_factory=lambda: int(OutputType.Nft), init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    mana: int = field(metadata=config(
        encoder=str
    ))
    nft_id: HexStr
    unlock_conditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition,
                                  StorageDepositReturnUnlockCondition, TimelockUnlockCondition]] = field(
        metadata=config(
            decoder=deserialize_unlock_conditions
        ))
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = field(default=None,
                                                                   metadata=config(
                                                                       decoder=deserialize_features
                                                                   ))
    immutable_features: Optional[List[Union[
        IssuerFeature, MetadataFeature]]] = field(default=None,
                                                  metadata=config(
                                                      decoder=deserialize_features
                                                  ))


@json
@dataclass
class DelegationOutput:
    """An output which delegates its contained IOTA coins as voting power to a validator.

    Attributes:
        amount: The amount of IOTA coins held by the output.
        delegated_amount: The amount of delegated IOTA coins.
        delegation_id: Unique identifier of the Delegation Output
        validator_address: The Account Address of the validator to which this output is delegating.
        start_epoch: The index of the first epoch for which this output delegates.
        end_epoch: The index of the last epoch for which this output delegates.
        unlock_conditions: Define how the output can be unlocked in a transaction.
        type: The type of output.
    """
    type: int = field(default_factory=lambda: int(
        OutputType.Delegation), init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    delegated_amount: int = field(metadata=config(
        encoder=str
    ))
    delegation_id: HexStr
    validator_address: AccountAddress
    start_epoch: EpochIndex
    end_epoch: EpochIndex
    unlock_conditions: List[AddressUnlockCondition] = field(metadata=config(
        decoder=deserialize_unlock_conditions
    ))


Output: TypeAlias = Union[BasicOutput,
                          AccountOutput,
                          AnchorOutput,
                          FoundryOutput,
                          NftOutput,
                          DelegationOutput]


def deserialize_output(d: Dict[str, Any]) -> Output:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    output_type = d['type']
    if output_type == OutputType.Basic:
        return BasicOutput.from_dict(d)
    if output_type == OutputType.Account:
        return AccountOutput.from_dict(d)
    if output_type == OutputType.Anchor:
        return AnchorOutput.from_dict(d)
    if output_type == OutputType.Foundry:
        return FoundryOutput.from_dict(d)
    if output_type == OutputType.Nft:
        return NftOutput.from_dict(d)
    if output_type == OutputType.Delegation:
        return DelegationOutput.from_dict(d)
    raise Exception(f'invalid output type: {output_type}')


def deserialize_outputs(dicts: List[Dict[str, Any]]) -> List[Output]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_output, dicts))
