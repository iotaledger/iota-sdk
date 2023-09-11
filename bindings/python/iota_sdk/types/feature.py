# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum

from dataclasses import dataclass, field
from typing import List

from iota_sdk.types.address import Ed25519Address, AccountAddress, NFTAddress
from iota_sdk.types.common import HexStr, json


class FeatureType(IntEnum):
    """Types of features.

    Attributes:
        Sender (0): The sender feature.
        Issuer (1): The issuer feature.
        Metadata (2): The metadata feature.
        Tag (3): The tag feature.
        BlockIssuer (4): The block issuer feature.
        Staking (5): The staking feature.
    """
    Sender = 0
    Issuer = 1
    Metadata = 2
    Tag = 3
    BlockIssuer = 4
    Staking = 5


@json
@dataclass
class Feature():
    """Base class of a feature.
    """
    type: int


@json
@dataclass
class SenderFeature(Feature):
    """Identifies the validated sender of an output.
    Attributes:
        address: A given sender address.
    """
    address: Ed25519Address | AccountAddress | NFTAddress
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Sender),
        init=False)


@json
@dataclass
class IssuerFeature(Feature):
    """Identifies the validated issuer of the UTXO state machine.
    Attributes:
        address: A given issuer address.
    """
    address: Ed25519Address | AccountAddress | NFTAddress
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Issuer),
        init=False)


@json
@dataclass
class MetadataFeature(Feature):
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
class TagFeature(Feature):
    """Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
    Attributes:
        tag: A hex encoded tag used to index the output.
    """
    tag: HexStr
    type: int = field(default_factory=lambda: int(FeatureType.Tag), init=False)


@json
@dataclass
class BlockIssuer(Feature):
    """Contains the public keys to verify block signatures and allows for unbonding the issuer deposit.
    Attributes:
        expiry_slot: The slot index at which the Block Issuer Feature expires and can be removed.
        public_keys: The Block Issuer Keys.
    """
    # TODO Replace with a proper SlotIndex type
    expiry_slot: str
    # TODO Replace with a list of PublicKey types
    public_keys: List[HexStr]
    type: int = field(
        default_factory=lambda: int(
            FeatureType.BlockIssuer),
        init=False)


@json
@dataclass
class StakingFeature(Feature):
    """Stakes IOTA coins to become eligible for committee selection, validate the network and receive Mana rewards.
    Attributes:
        staked_amount: The amount of IOTA coins that are locked and staked in the containing account.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
        start_epoch: The epoch index in which the staking started.
        end_epoch: The epoch index in which the staking ends.
    """
    staked_amount: str
    fixed_cost: str
    # TODO Replace with an EpochIndex type
    start_epoch: HexStr
    # TODO Replace with an EpochIndex type
    end_epoch: HexStr
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Staking),
        init=False)
