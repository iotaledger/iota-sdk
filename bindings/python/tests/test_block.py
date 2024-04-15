# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from iota_sdk import Block, ProtocolParameters, Utils

protocol_params_json = {}
with open('../../sdk/tests/types/fixtures/protocol_parameters.json', "r", encoding="utf-8") as params:
    protocol_params_json = json.load(params)


def test_basic_block_tagged_data_payload():
    basic_block_tagged_data_payload_json = {}
    with open('../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json', "r", encoding="utf-8") as payload:
        basic_block_tagged_data_payload_json = json.load(payload)
    block = Block.from_dict(basic_block_tagged_data_payload_json['block'])
    protocol_params = ProtocolParameters.from_dict(
        protocol_params_json['params'])
    expected_id = basic_block_tagged_data_payload_json['id']
    assert block.id(protocol_params) == expected_id
    assert Utils.block_work_score(
        block, protocol_params.work_score_parameters) == basic_block_tagged_data_payload_json['workScore']


# def test_basic_block_transaction_payload():
#     basic_block_transaction_payload_json = {}
#     with open('../../sdk/tests/types/fixtures/basic_block_transaction_payload.json', "r", encoding="utf-8") as payload:
#         basic_block_transaction_payload_json = json.load(payload)
#     block = Block.from_dict(basic_block_transaction_payload_json['block'])
#     protocol_params = ProtocolParameters.from_dict(
#         protocol_params_json['params'])
#     expected_id = basic_block_transaction_payload_json['id']
#     assert block.id(protocol_params) == expected_id


# def test_validation_block():
#     validation_block_json = {}
#     with open('../../sdk/tests/types/fixtures/validation_block.json', "r", encoding="utf-8") as payload:
#         validation_block_json = json.load(payload)
#     block = Block.from_dict(validation_block_json['block'])
#     protocol_params = ProtocolParameters.from_dict(
#         protocol_params_json['params'])
#     expected_id = validation_block_json['id']
#     assert block.id(protocol_params) == expected_id
