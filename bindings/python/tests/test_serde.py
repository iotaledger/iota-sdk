# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import Features, Unlocks, BasicOutput, SendParams


def test_feature():
    features_dict = {
        "tag": "0x426c61",
        "metadata": "0x426c61",
        "issuer": "issuer",
        "sender": "sender"
    }

    features = Features.from_dict(features_dict)
    assert features.to_dict() == features_dict

    opt_features_dict = {
        "tag": "0x426c61",
        "sender": "sender"
    }

    opt_features = Features.from_dict(opt_features_dict)
    assert opt_features.to_dict() == opt_features_dict


def test_unlocks():
    unlocks_dict = {
        "expirationSlotIndex": "1",
        "timelockSlotIndex": "2"
    }

    unlocks = Unlocks.from_dict(unlocks_dict)
    assert unlocks.expiration_slot_index == 1
    assert unlocks.timelock_slot_index == 2
    assert unlocks.to_dict() == unlocks_dict

    opt_unlocks_dict = {
        "expirationSlotIndex": "1"
    }

    opt_unlocks = Unlocks.from_dict(opt_unlocks_dict)
    assert opt_unlocks.timelock_slot_index is None
    assert opt_unlocks.to_dict() == opt_unlocks_dict


def test_outputs():
    basic_output_dict = {
        "type": 3,
        "mana": "999500700",
        "amount": "999500701",
        "unlockConditions": [
            {
                "type": 1,
                "returnAddress": {
                    "type": 0,
                    "pubKeyHash": "0x8f463f0c57b86cf52cc69542fb43a2ec87f83b9c47493cce04c1a4616716bed0"
                },
                "amount": "57600"
            }
        ]
    }
    basic_output = BasicOutput.from_dict(basic_output_dict)
    assert basic_output.mana == 999500700
    assert basic_output.amount == 999500701
    assert basic_output.unlock_conditions[0].amount == 57600


def test_send_params():
    send_params_dict = {
        "address": "",
        "amount": "123",
        "returnAddress": "abc"
    }

    send_params = SendParams.from_dict(send_params_dict)
    assert send_params.amount == 123
    assert send_params.return_address == "abc"
    assert send_params.expiration is None
    assert send_params.to_dict() == send_params_dict
