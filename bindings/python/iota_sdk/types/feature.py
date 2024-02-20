# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from typing import Dict, List, TypeAlias, Union, Any
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.address import Address, deserialize_address
from iota_sdk.types.block_issuer_key import BlockIssuerKey
from iota_sdk.types.common import EpochIndex, HexStr, hex_str_decoder, json, SlotIndex


class FeatureType(IntEnum):
    """Types of features.

    Attributes:
        Sender (0): The sender feature.
        Issuer (1): The issuer feature.
        Metadata (2): The metadata feature.
        StateMetadata (3): The state metadata feature.
        Tag (4): The tag feature.
        NativeToken (5): The native token feature.
        BlockIssuer (6): The block issuer feature.
        Staking (7): The staking feature.
    """
    Sender = 0
    Issuer = 1
    Metadata = 2
    StateMetadata = 3
    Tag = 4
    NativeToken = 5
    BlockIssuer = 6
    Staking = 7


@json
@dataclass
class SenderFeature:
    """Identifies the validated sender of an output.
    Attributes:
        address: A given sender address.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Sender),
        init=False)
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class IssuerFeature:
    """Identifies the validated issuer of the UTXO state machine.
    Attributes:
        address: A given issuer address.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Issuer),
        init=False)
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class MetadataFeature:
    """Defines metadata, arbitrary binary data, that will be stored in the output.
    Attributes:
        entries: A key-value map where the keys are graphic ASCII strings and the values hex-encoded binary data.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Metadata),
        init=False)
    entries: Dict[str, HexStr]


@json
@dataclass
class StateMetadataFeature:
    """A Metadata Feature that can only be changed by the State Controller.
    Attributes:
        entries: A key-value map where the keys are graphic ASCII strings and the values hex-encoded binary data.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.StateMetadata),
        init=False)
    entries: Dict[str, HexStr]


@json
@dataclass
class TagFeature:
    """Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
    Attributes:
        tag: A hex encoded tag used to index the output.
    """
    type: int = field(default_factory=lambda: int(FeatureType.Tag), init=False)
    tag: HexStr


@json
@dataclass
class NativeTokenFeature:
    """Contains a native token.
        id: The unique identifier of the native token.
        amount: The amount of native tokens.
    """
    type: int = field(default_factory=lambda: int(
        FeatureType.NativeToken), init=False)
    id: HexStr
    amount: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))


@json
@dataclass
class BlockIssuerFeature:
    """Contains the public keys to verify block signatures and allows for unbonding the issuer deposit.
    Attributes:
        expiry_slot: The slot index at which the Block Issuer Feature expires and can be removed.
        block_issuer_keys: The Block Issuer Keys.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.BlockIssuer),
        init=False)
    expiry_slot: SlotIndex
    block_issuer_keys: List[BlockIssuerKey]


@json
@dataclass
class StakingFeature:
    """Stakes IOTA coins to become eligible for committee selection, validate the network and receive Mana rewards.
    Attributes:
        staked_amount: The amount of IOTA coins that are locked and staked in the containing account.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
        start_epoch: The epoch index in which the staking started.
        end_epoch: The epoch index in which the staking ends.
    """
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Staking),
        init=False)
    staked_amount: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))
    start_epoch: EpochIndex
    end_epoch: EpochIndex


Feature: TypeAlias = Union[SenderFeature, IssuerFeature,
                           MetadataFeature, StateMetadataFeature, TagFeature, NativeTokenFeature, BlockIssuerFeature, StakingFeature]


# pylint: disable=too-many-return-statements
def deserialize_feature(d: Dict[str, Any]) -> Feature:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    feature_type = d['type']
    if feature_type == FeatureType.Sender:
        return SenderFeature.from_dict(d)
    if feature_type == FeatureType.Issuer:
        return IssuerFeature.from_dict(d)
    if feature_type == FeatureType.Metadata:
        return MetadataFeature.from_dict(d)
    if feature_type == FeatureType.StateMetadata:
        return StateMetadataFeature.from_dict(d)
    if feature_type == FeatureType.Tag:
        return TagFeature.from_dict(d)
    if feature_type == FeatureType.NativeToken:
        return NativeTokenFeature.from_dict(d)
    if feature_type == FeatureType.BlockIssuer:
        return BlockIssuerFeature.from_dict(d)
    if feature_type == FeatureType.Staking:
        return StakingFeature.from_dict(d)
    raise Exception(f'invalid feature type: {feature_type}')


def deserialize_features(dicts: List[Dict[str, Any]]) -> List[Feature]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_feature, dicts))
