# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from typing import get_args
import pytest
from iota_sdk import BasicBlock, BlockType, Payload, PayloadType, ProtocolParameters, SignedBlock, Utils, ValidationBlock

basic_block_tagged_data_payload_json = {}
with open('../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json', "r", encoding="utf-8") as json_file:
    basic_block_tagged_data_payload_json = json.load(json_file)

basic_block_transaction_payload_json = {}
with open('../../sdk/tests/types/fixtures/basic_block_transaction_payload.json', "r", encoding="utf-8") as json_file:
    signed_block_transaction_payload_json = json.load(json_file)

validation_block_json = {}
with open('../../sdk/tests/types/fixtures/validation_block.json', "r", encoding="utf-8") as json_file:
    validation_block_json = json.load(json_file)

protocol_params_json = {}
with open('../../sdk/tests/types/fixtures/protocol_parameters.json', "r", encoding="utf-8") as json_file:
    protocol_params_json = json.load(json_file)

def test_basic_block_tagged_data_payload():
    signed_block_dict = basic_block_tagged_data_payload_json['block']
    signed_block = SignedBlock.from_dict(signed_block_dict)
    assert signed_block.to_dict() == signed_block_dict

    assert isinstance(signed_block.body, BasicBlock)
    assert signed_block.body.type == BlockType.Basic
    assert signed_block.body.max_burned_mana == 864

    assert isinstance(signed_block.body.payload, get_args(Payload))
    assert signed_block.body.payload.type == PayloadType.TaggedData

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)

    expected_id = basic_block_tagged_data_payload_json['id']
    assert signed_block.id(protocol_params) == expected_id


def test_basic_block_transaction_payload():
    signed_block_dict = basic_block_transaction_payload_json['block']
    signed_block = SignedBlock.from_dict(signed_block_dict)
    assert signed_block.to_dict() == signed_block_dict

    assert isinstance(signed_block.body, BasicBlock)
    assert signed_block.body.type == BlockType.Basic
    assert signed_block.body.max_burned_mana == 119

    assert isinstance(signed_block.body.payload, get_args(Payload))
    assert signed_block.body.payload.type == PayloadType.SignedTransaction

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)

    expected_id = basic_block_transaction_payload_json['id']
    assert signed_block.id(protocol_params) == expected_id

def test_validation_block():
    signed_block_dict = validation_block_json['block']
    signed_block = SignedBlock.from_dict(signed_block_dict)
    assert signed_block.to_dict() == signed_block_dict

    assert isinstance(signed_block.body, ValidationBlock)
    assert signed_block.body.type == BlockType.Validation

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)

    expected_id = validation_block_json['id']
    assert signed_block.id(protocol_params) == expected_id
