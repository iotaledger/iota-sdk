# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from iota_sdk import call_wallet_method
from iota_sdk.common import custom_encoder


def _call_wallet_method_routine(func):
    """The routine of dump json string and call call_wallet_method().
    """
    def wrapper(*args, **kwargs):
        message = custom_encoder(func, *args, **kwargs)
        # Send message to the Rust library
        response = call_wallet_method(args[0].handle, message)

        json_response = json.loads(response)

        if "type" in json_response:
            if json_response["type"] == "error" or json_response["type"] == "panic":
                raise WalletError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        return response
    return wrapper


class WalletError(Exception):
    """A wallet error."""
