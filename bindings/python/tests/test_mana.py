# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# import json
# from iota_sdk import Utils, ProtocolParameters, deserialize_output


# https://github.com/iotaledger/tips/blob/tip45/tips/TIP-0045/tip-0045.md#potential-and-stored-mana


# def test_output_mana():
#     protocol_params_json = {}

#     with open('../../sdk/tests/types/fixtures/protocol_parameters.json', "r", encoding="utf-8") as json_file:
#         protocol_params_json = json.load(json_file)

#     protocol_params_dict = protocol_params_json['params']
#     protocol_parameters = ProtocolParameters.from_dict(protocol_params_dict)
#     output_dict = {
#         "type": 0,
#         "amount": "100000",
#         "mana": "4000",
#         "unlockConditions": [
#             {
#                 "type": 0,
#                 "address": {
#                     "type": 0,
#                     "pubKeyHash": "0xed1484f4d1f7d8c037087fed661dd92faccae1eed3c01182d6fdd6828cea144a"
#                 }
#             }
#         ]
#     }
#     creation_slot = 5
#     target_slot = 5000000
#     output = deserialize_output(output_dict)
#     decayed_mana = Utils.output_mana_with_decay(
#         output, creation_slot, target_slot, protocol_parameters)
#     assert decayed_mana.stored == 2272
#     assert decayed_mana.potential == 2502459

#     decayed_stored_mana = Utils.mana_with_decay(
#         output.mana, creation_slot, target_slot, protocol_parameters)
#     assert decayed_stored_mana == 2272

#     # storage deposit doesn't generate mana
#     minimum_output_amount = Utils.compute_minimum_output_amount(
#         output, protocol_parameters.storage_score_parameters)

#     decayed_potential_mana = Utils.generate_mana_with_decay(
#         output.amount - minimum_output_amount, creation_slot, target_slot, protocol_parameters)
#     assert decayed_potential_mana == 2502459
