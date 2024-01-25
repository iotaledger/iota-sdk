# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from iota_sdk import Block, ProtocolParameters

protocol_params_json = {}
with open('../../sdk/tests/types/fixtures/protocol_parameters.json', "r", encoding="utf-8") as json_file:
    protocol_params_json = json.load(json_file)


def test_basic_block_tagged_data_payload():
    basic_block_tagged_data_payload_json = {}
    with open('../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json', "r", encoding="utf-8") as json_file:
        basic_block_tagged_data_payload_json = json.load(json_file)

    block_dict = basic_block_tagged_data_payload_json['block']
    block = Block.from_dict(block_dict)
    assert block.to_dict() == block_dict

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)
    expected_id = basic_block_tagged_data_payload_json['id']
    assert block.id(protocol_params) == expected_id


def test_basic_block_transaction_payload():
    basic_block_transaction_payload_json = {}
    with open('../../sdk/tests/types/fixtures/basic_block_transaction_payload.json', "r", encoding="utf-8") as json_file:
        basic_block_transaction_payload_json = json.load(json_file)

    block_dict = basic_block_transaction_payload_json['block']
    block = Block.from_dict(block_dict)
    assert block.to_dict() == block_dict

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)
    expected_id = basic_block_transaction_payload_json['id']
    assert block.id(protocol_params) == expected_id


def test_validation_block():
    validation_block_json = {}
    with open('../../sdk/tests/types/fixtures/validation_block.json', "r", encoding="utf-8") as json_file:
        validation_block_json = json.load(json_file)

    block_dict = validation_block_json['block']
    block = Block.from_dict(block_dict)
    assert block.to_dict() == block_dict

    protocol_params_dict = protocol_params_json['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)
    expected_id = validation_block_json['id']
    assert block.id(protocol_params) == expected_id
