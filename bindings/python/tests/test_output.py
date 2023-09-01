
# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import BasicOutput, AliasOutput, FoundryOutput, NftOutput, IssuerFeature, MetadataFeature
from dacite import from_dict


def test_feature():
    feature_dict = {
        "type": 2,
        "data": "0x426c61"
    }
    metadata_feature = from_dict(MetadataFeature, feature_dict)
    assert metadata_feature.to_dict() == feature_dict

    issuer_dict = {
        "type": 1,
        "address": {
            "type": 0,
            "pubKeyHash": "0xd970bcafdc18859b3fd3380f759bb520c36a29bd682b130623c6604ce3526ea1"
        }
    }
    issuer_feature = IssuerFeature.from_dict(issuer_dict)
    assert issuer_feature.to_dict() == issuer_dict


def test_output():
    basic_output_dict = {
        "type": 3,
        "amount": "999500700",
        "unlockConditions": [
            {
                "type": 0,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x89e302f66c5775df0d2018748abdde18cde6351cea2006eeed63f946276b48e2"
                }
            }
        ]
    }
    basic_output = BasicOutput.from_dict(basic_output_dict)
    assert basic_output.to_dict() == basic_output_dict

    basic_output_dict = {
        "type": 3,
        "amount": "57600",
        "nativeTokens": [
            {
                "id": "0x086326539ce1b78eb606a75950f31698ddcb51200b4ee6e870050e6ef658cd3bab0100000000",
                "amount": "0x32"
            }
        ],
        "unlockConditions": [
            {
                "type": 0,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0xf8ba764448d689422aa59a5c5dc97108450a29cb8956208f631ab4a82338468a"
                }
            },
            {
                "type": 1,
                "returnAddress": {
                    "type": 0,
                    "pubKeyHash": "0x8f463f0c57b86cf52cc69542fb43a2ec87f83b9c47493cce04c1a4616716bed0"
                },
                "amount": "57600"
            },
            {
                "type": 3,
                "returnAddress": {
                    "type": 0,
                    "pubKeyHash": "0x8f463f0c57b86cf52cc69542fb43a2ec87f83b9c47493cce04c1a4616716bed0"
                },
                "unixTime": 1659119101
            }
        ]
    }
    basic_output = BasicOutput.from_dict(basic_output_dict)
    assert basic_output.to_dict() == basic_output_dict

    basic_output_dict = {
        "type": 3,
        "amount": "50100",
        "nativeTokens": [
            {
                "id": "0x087f3221adb3be9ef74a69595ef282b4ca47fd98b6bf1142e7d8f9f7b265efeedc0100000000",
                "amount": "0x1"
            }
        ],
        "unlockConditions": [
            {
                "type": 0,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x3d3076218d7f0c7a1489b73ab99697ff698a7d0e89b7f5fbb4c1df7059b78679"
                }
            },
            {
                "type": 2,
                "unixTime": 1661850262
            }
        ]
    }
    basic_output = BasicOutput.from_dict(basic_output_dict)
    assert basic_output.to_dict() == basic_output_dict

    alias_output_dict = {
        "type": 4,
        "amount": "168200",
        "aliasId": "0x8d073d15074834785046d9cacec7ac4d672dcb6dad342624a936f3c4334520f1",
        "stateIndex": 4,
        "stateMetadata": "0x14bd8ce73814dfe5d6f30f65a11bfd6d0b9e5d29c90aff9d71ec4b3d3a2984386a312295fc8b79cd",
        "foundryCounter": 0,
        "unlockConditions": [
            {
                "type": 4,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x1f964c683db3072db2ad26ec4b4bee69fb4224755e65566e284fc2aac057edbc"
                }
            },
            {
                "type": 5,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x1f964c683db3072db2ad26ec4b4bee69fb4224755e65566e284fc2aac057edbc"
                }
            }
        ],
        "features": [
            {
                "type": 0,
                "address": {
                    "type": 8,
                    "aliasId": "0x8d073d15074834785046d9cacec7ac4d672dcb6dad342624a936f3c4334520f1"
                }
            }
        ]
    }
    alias_output = AliasOutput.from_dict(alias_output_dict)
    assert alias_output.to_dict() == alias_output_dict

    alias_output_dict = {
        "type": 4,
        "amount": "55100",
        "aliasId": "0x5380cce0ac342b8fa3e9c4f46d5b473ee9e824f0017fe43682dca77e6b875354",
        "stateIndex": 2,
        "stateMetadata": "0x",
        "foundryCounter": 1,
        "unlockConditions": [
            {
                "type": 4,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0xc5976c01059227e9246686f138b29d13c3a85efd8a2154729dce23a3dfd52119"
                }
            },
            {
                "type": 5,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0xc5976c01059227e9246686f138b29d13c3a85efd8a2154729dce23a3dfd52119"
                }
            }
        ],
        "immutableFeatures": [
            {
                "type": 1,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x8262e2afc09fb7729b00b5d3c86e492b82ed7d8d9cb3f48ca40e9deeda077875"
                }
            },
            {
                "type": 2,
                "data": "0x6e6f2d6d65746164617461"
            }
        ]
    }
    alias_output = AliasOutput.from_dict(alias_output_dict)
    assert alias_output.to_dict() == alias_output_dict

    foundry_output_dict = {
        "type": 5,
        "amount": "54700",
        "serialNumber": 1,
        "tokenScheme": {
            "type": 0,
            "mintedTokens": "0x539",
            "meltedTokens": "0x0",
            "maximumSupply": "0x539"
        },
        "unlockConditions": [
            {
                "type": 6,
                "address": {
                    "type": 8,
                    "aliasId": "0xf89cfa69c0dd2946ae207f2fcae34b1b1ffa5cefdb5d6fd9ccaa068629803ff5"
                }
            }
        ],
        "immutableFeatures": [
            {
                "type": 2,
                "data": "0x4c9385555f70b41d47f000c08dbe6913"
            }
        ]
    }
    foundry_output = FoundryOutput.from_dict(foundry_output_dict)
    assert foundry_output.to_dict() == foundry_output_dict

    nft_output_dict = {
        "type": 6,
        "amount": "47800",
        "nftId": "0x90e84936bd0cffd1595d2a58f63b1a8d0d3e333ed893950a5f3f0043c6e59ec1",
        "unlockConditions": [
            {
                "type": 0,
                "address": {
                    "type": 0,
                    "pubKeyHash": "0x6d09d800c9221d818bbae5df148f4f7b1fe6b7a424f60702e5498a6ee75a568c"
                }
            }
        ],
        "features": [
            {
                "type": 2,
                "data": "0x547275657d"
            }
        ],
        "immutableFeatures": [
            {
                "type": 2,
                "data": "0x7b69735f6e66743a"
            }
        ]
    }
    nft_output = NftOutput.from_c, nft_output_dict)
    assert nft_output.to_dict() == nft_output_dict
