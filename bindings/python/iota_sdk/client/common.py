# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import call_client_method
from json import dumps, loads


def call_method_routine(func):
    """The routine of dump json string and call call_client_method()
    """
    def wrapper(*args, **kwargs):
        message = func(*args, **kwargs)
        message = dumps(message)

        # Send message to the Rust library
        response = call_client_method(args[0].handle, message)

        json_response = loads(response)

        if "type" in json_response:
            if json_response["type"] == "error":
                raise ClientError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        else:
            return response
    return wrapper

class ClientError(Exception):
    """client error"""
    pass
