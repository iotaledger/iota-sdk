# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Dict, Optional, List, Union
from dacite import from_dict
from iota_sdk.types.common import HexStr
from iota_sdk.types.feature import SenderFeature, IssuerFeature, MetadataFeature, TagFeature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition, ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition


class OutputType(IntEnum):
    """Output types.

    Attributes:
        Treasury (2): A treasury output.
        Basic (3): A basic output.
        Alias (4): An alias output.
        Foundry (5): A foundry output.
        Nft (6): An NFT output.
    """
    Treasury = 2
    Basic = 3
    Alias = 4
    Foundry = 5
    Nft = 6


@dataclass
class Output():
    """An output in a UTXO ledger.
    """
    type: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'unlockConditions' in config:
            config['unlockConditions'] = list(map(
                lambda x: x.as_dict(), config['unlockConditions']))
        if 'nativeTokens' in config:
            config['nativeTokens'] = list(map(
                lambda x: x.__dict__, config['nativeTokens']))
        if 'features' in config:
            config['features'] = list(map(
                lambda x: x.as_dict(), config['features']))
        if 'immutableFeatures' in config:
            config['immutableFeatures'] = list(map(
                lambda x: x.as_dict(), config['immutableFeatures']))
        if 'tokenScheme' in config:
            config['tokenScheme'] = config['tokenScheme'].__dict__

        return config


@dataclass
class TreasuryOutput(Output):
    """Describes a treasury output.
    Attributes:
        amount :
            The base coin amount of the output.
        type :
            The type of output.
    """
    amount: str
    type: int = field(
        default_factory=lambda: int(
            OutputType.Treasury),
        init=False)


@dataclass
class BasicOutput(Output):
    """Describes a basic output.
    Attributes:
        amount :
            The base coin amount of the output.
        unlockConditions :
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        nativeTokens :
            Native tokens added to the new output.
        type :
            The type of output.
    """
    amount: str
    unlockConditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                           TimelockUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = None
    nativeTokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Basic),
        init=False)


@dataclass
class AliasOutput(Output):
    """Describes an alias output.
    Attributes:
        amount :
            The base coin amount of the output.
        unlockConditions :
            The conditions to unlock the output.
        aliasId :
            The alias ID if it's an alias output.
        stateIndex :
            A counter that must increase by 1 every time the alias is state transitioned.
        stateMetadata :
            Metadata that can only be changed by the state controller.
        foundryCounter :
            A counter that denotes the number of foundries created by this alias account.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        nativeTokens :
            Native tokens added to the new output.
        immutableFeatures :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    amount: str
    aliasId: HexStr
    stateIndex: int
    foundryCounter: int
    unlockConditions: List[Union[StateControllerAddressUnlockCondition,
                           GovernorAddressUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature]]] = None
    immutableFeatures: Optional[List[Union[IssuerFeature,
                                     MetadataFeature]]] = None
    stateMetadata: Optional[HexStr] = None
    nativeTokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Alias),
        init=False)


@dataclass
class FoundryOutput(Output):
    """Describes a foundry output.
    Attributes:
        amount :
            The base coin amount of the output.
        unlockConditions :
            The conditions to unlock the output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        nativeTokens :
            Native tokens added to the new output.
        immutableFeatures :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        serialNumber :
            The serial number of the foundry with respect to the controlling alias.
        tokenScheme :
            Defines the supply control scheme of the tokens controlled by the foundry. Currently only a simple scheme is supported.
        type :
            The type of output.
    """
    amount: str
    serialNumber: int
    tokenScheme: SimpleTokenScheme
    unlockConditions: List[ImmutableAliasAddressUnlockCondition]
    features: Optional[List[MetadataFeature]] = None
    immutableFeatures: Optional[List[MetadataFeature]] = None
    nativeTokens: Optional[List[NativeToken]] = None
    type: int = field(
        default_factory=lambda: int(
            OutputType.Foundry),
        init=False)


@dataclass
class NftOutput(Output):
    """Describes an NFT output.
    Attributes:
        amount :
            The base coin amount of the output.
        unlockConditions :
            The conditions to unlock the output.
        nftId :
            The NFT ID if it's an NFT output.
        features :
            Features that add utility to the output but do not impose unlocking conditions.
        nativeTokens :
            Native tokens added to the new output.
        immutableFeatures :
            Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.
        type :
            The type of output.
    """
    amount: str
    nftId: HexStr
    unlockConditions: List[Union[AddressUnlockCondition, ExpirationUnlockCondition,
                           StorageDepositReturnUnlockCondition, TimelockUnlockCondition]]
    features: Optional[List[Union[SenderFeature,
                            MetadataFeature, TagFeature]]] = None
    immutableFeatures: Optional[List[Union[
        IssuerFeature, MetadataFeature]]] = None
    nativeTokens: Optional[List[NativeToken]] = None
    type: int = field(default_factory=lambda: int(OutputType.Nft), init=False)


@dataclass
class OutputMetadata:
    """Metadata about an output.

    Attributes:
        blockId: The ID of the block in which the output appeared in.
        transactionId: The ID of the transaction in which the output was created.
        outputIndex: The index of the output within the corresponding transaction.
        isSpent: Whether the output is already spent.
        milestoneIndexBooked: The index of the milestone which booked/created the output.
        milestoneTimestampBooked: The timestamp of the milestone which booked/created the output.
        ledgerIndex: The current ledger index.
        milestoneIndexSpent: The index of the milestone which spent the output.
        milestoneTimestampSpent: The timestamp of the milestone which spent the output.
        transactionIdSpent: The ID of the transaction that spent the output.
    """
    blockId: HexStr
    transactionId: HexStr
    outputIndex: int
    isSpent: bool
    milestoneIndexBooked: int
    milestoneTimestampBooked: int
    ledgerIndex: int
    milestoneIndexSpent: Optional[int] = None
    milestoneTimestampSpent: Optional[int] = None
    transactionIdSpent: Optional[HexStr] = None

    # pylint: disable=redefined-builtin
    @classmethod
    def from_dict(cls, dict: Dict) -> OutputMetadata:
        """Converts a dict to a OutputMetadata
        """
        obj = cls.__new__(cls)
        super(OutputMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj

    def as_dict(self):
        """Converts this object to a dict.
        """
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class OutputWithMetadata:
    """An output with its metadata.

    Attributes:
        metadata: The `OutputMetadata` object that belongs to `output`.
        output: An `Output` object.
    """

    metadata: OutputMetadata
    output: Union[AliasOutput, FoundryOutput, NftOutput, BasicOutput]

    # pylint: disable=redefined-builtin
    @classmethod
    def from_dict(cls, dict: Dict) -> OutputWithMetadata:
        """Creates an output with metadata instance from the dict object.
        """
        obj = cls.__new__(cls)
        super(OutputWithMetadata, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {}

        config['metadata'] = self.metadata.__dict__
        config['output'] = self.output.as_dict()

        return config


def output_from_dict(
        output: Dict[str, any]) -> Union[TreasuryOutput, BasicOutput, AliasOutput, FoundryOutput, NftOutput, Output]:
    """
    Takes a dictionary as input and returns an instance of a specific
    output class based on the value of the 'type' key in the dictionary.

    Arguments:

    * `output`: The `output` parameter is a dictionary that contains information about the output. It is
    expected to have a key called 'type' which specifies the type of the output. The value of 'type'
    should be one of the values defined in the `OutputType` enum.
    """
    output_type = OutputType(output['type'])

    if output_type == OutputType.Treasury:
        return from_dict(TreasuryOutput, output)
    if output_type == OutputType.Basic:
        return from_dict(BasicOutput, output)
    if output_type == OutputType.Alias:
        return from_dict(AliasOutput, output)
    if output_type == OutputType.Foundry:
        return from_dict(FoundryOutput, output)
    if output_type == OutputType.Nft:
        return from_dict(NftOutput, output)

    return from_dict(Output, output)
