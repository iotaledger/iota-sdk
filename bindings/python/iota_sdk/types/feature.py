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
        Tag (3): The tag feature.
        NativeToken (4): The native token feature.
        BlockIssuer (5): The block issuer feature.
        Staking (6): The staking feature.
    """
    Sender = 0
    Issuer = 1
    Metadata = 2
    Tag = 3
    NativeToken = 4
    BlockIssuer = 5
    Staking = 6


@json
@dataclass
class SenderFeature:
    """Identifies the validated sender of an output.
    Attributes:
        address: A given sender address.
    """
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Sender),
        init=False)


@json
@dataclass
class IssuerFeature:
    """Identifies the validated issuer of the UTXO state machine.
    Attributes:
        address: A given issuer address.
    """
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Issuer),
        init=False)


@json
@dataclass
class MetadataFeature:
    """Defines metadata, arbitrary binary data, that will be stored in the output.
    Attributes:
        data: Some hex encoded metadata.
    """
    data: HexStr
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Metadata),
        init=False)


@json
@dataclass
class TagFeature:
    """Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
    Attributes:
        tag: A hex encoded tag used to index the output.
    """
    tag: HexStr
    type: int = field(default_factory=lambda: int(FeatureType.Tag), init=False)


@json
@dataclass
class NativeTokenFeature:
    """Contains a native token.
        id: The unique identifier of the native token.
        amount: The amount of native tokens.
    """
    id: HexStr
    amount: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    type: int = field(default_factory=lambda: int(
        FeatureType.NativeToken), init=False)


@json
@dataclass
class BlockIssuerFeature:
    """Contains the public keys to verify block signatures and allows for unbonding the issuer deposit.
    Attributes:
        expiry_slot: The slot index at which the Block Issuer Feature expires and can be removed.
        block_issuer_keys: The Block Issuer Keys.
    """
    expiry_slot: SlotIndex
    block_issuer_keys: List[BlockIssuerKey]
    type: int = field(
        default_factory=lambda: int(
            FeatureType.BlockIssuer),
        init=False)


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
    staked_amount: int = field(metadata=config(
        encoder=str
    ))
    fixed_cost: int = field(metadata=config(
        encoder=str
    ))
    start_epoch: EpochIndex
    end_epoch: EpochIndex
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Staking),
        init=False)


Feature: TypeAlias = Union[SenderFeature, IssuerFeature,
                           MetadataFeature, TagFeature, NativeTokenFeature, BlockIssuerFeature, StakingFeature]


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
