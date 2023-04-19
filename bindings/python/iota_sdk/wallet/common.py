# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import call_wallet_method
import json
from json import dumps


def _call_method_routine(func):
    """The routine of dump json string and call call_wallet_method()
    """
    def wrapper(*args, **kwargs):
        message = func(*args, **kwargs)
        message = dumps(message)

        # Send message to the Rust library
        response = call_wallet_method(args[0].handle, message)

        json_response = json.loads(response)

        if "type" in json_response:
            if json_response["type"] == "error":
                raise WalletError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        else:
            return response
    return wrapper

class WalletError(Exception):
    """wallet error"""
    pass