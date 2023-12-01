# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from dacite import from_dict
from iota_sdk import ProtocolParameters


protocol_params = {}
with open('../../sdk/tests/types/fixtures/protocol_parameters.json', "r", encoding="utf-8") as json_file:
    protocol_params = json.load(json_file)

def test_protocol_parameters():
    protocol_params_dict = protocol_params['params']
    protocol_params = ProtocolParameters.from_dict(protocol_params_dict)
    assert protocol_params.to_dict() == protocol_params_dict
