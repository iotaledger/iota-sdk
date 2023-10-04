# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Dict, Optional, List, TypeAlias, Union, Any
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.feature import deserialize_features, SenderFeature, IssuerFeature, MetadataFeature, TagFeature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import deserialize_unlock_conditions, AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition, ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAccountAddressUnlockCondition


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
    Delegation = 7


@json
@dataclass
class BaseOutput():
    """An output in a UTXO ledger.
    """
    type: int


@json
@dataclass
class BasicOutput(BaseOutput):
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
    unlock_conditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                                  TimelockUnlockCondition]] = field(metadata=config(
                                                                    decoder=deserialize_unlock_conditions
                                                                    ))
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = field(default=None,
                                                                   metadata=config(
                                                                       decoder=deserialize_features
                                                                   ))
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Basic),
        init=False)


@json
@dataclass
class AccountOutput(BaseOutput):
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
    state_index: int
    foundry_counter: int
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
    state_metadata: Optional[HexStr] = None
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Account),
        init=False)


@json
@dataclass
class FoundryOutput(BaseOutput):
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
    features: Optional[List[MetadataFeature]] = field(default=None,
                                                      metadata=config(
                                                          decoder=deserialize_features
                                                      ))
    immutable_features: Optional[List[MetadataFeature]] = field(default=None,
                                                                metadata=config(
                                                                    decoder=deserialize_features
                                                                ))
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Foundry),
        init=False)


@json
@dataclass
class NftOutput(BaseOutput):
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
    native_tokens: Optional[List[NativeToken]] = None
    type: int = field(default_factory=lambda: int(OutputType.Nft), init=False)


@json
@dataclass
class DelegationOutput(BaseOutput):
    """Describes a delegation output.
    Attributes:
        type :
            The type of output.
    """
    # TODO fields done in #1174
    type: int = field(default_factory=lambda: int(
        OutputType.Delegation), init=False)


Output: TypeAlias = Union[BasicOutput, AccountOutput,
                          FoundryOutput, NftOutput, DelegationOutput]


def deserialize_output(d: Dict[str, Any]) -> Output:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    output_type = dict['type']
    if output_type == OutputType.Basic:
        return BasicOutput.from_dict(d)
    if output_type == OutputType.Account:
        return AccountOutput.from_dict(d)
    if output_type == OutputType.Foundry:
        return FoundryOutput.from_dict(d)
    if output_type == OutputType.Nft:
        return NftOutput.from_dict(d)
    if output_type == OutputType.Delegation:
        return DelegationOutput.from_dict(d)
    raise Exception(f'invalid output type: {output_type}')


def deserialize_outputs(dicts: List[Dict[str, Any]]) -> List[Output]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_output, dicts))
