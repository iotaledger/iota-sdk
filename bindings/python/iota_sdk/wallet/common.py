# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import call_wallet_method
import humps
import json
from json import dumps, JSONEncoder
from enum import Enum


def _call_method_routine(func):
    """The routine of dump json string and call call_wallet_method().
    """
    def wrapper(*args, **kwargs):
        class MyEncoder(JSONEncoder):
            def default(self, obj):
                as_dict_method = getattr(obj, "as_dict", None)
                if callable(as_dict_method):
                    return obj.as_dict()
                if isinstance(obj, str):
                    return obj
                if isinstance(obj, Enum):
                    return obj.__dict__
                if isinstance(obj, dict):
                    return obj
                if hasattr(obj, "__dict__"):
                    obj_dict = obj.__dict__

                    items_method = getattr(self, "items", None)
                    if callable(items_method):
                        for k, v in obj_dict.items():
                            obj_dict[k] = dumps(v, cls=MyEncoder)
                            return obj_dict
                    return obj_dict
                return obj
        message = func(*args, **kwargs)

        for k, v in message.items():
            if not isinstance(v, str):
                message[k] = json.loads(dumps(v, cls=MyEncoder))

        def remove_none(obj):
            if isinstance(obj, (list, tuple, set)):
                return type(obj)(remove_none(x) for x in obj if x is not None)
            elif isinstance(obj, dict):
                return type(obj)((remove_none(k), remove_none(v))
                                 for k, v in obj.items() if k is not None and v is not None)
            else:
                return obj
        message_null_filtered = remove_none(message)
        message = dumps(humps.camelize(message_null_filtered))
        # Send message to the Rust library
        response = call_wallet_method(args[0].handle, message)

        json_response = json.loads(response)

        if "type" in json_response:
            if json_response["type"] == "error" or json_response["type"] == "panic":
                raise WalletError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        else:
            return response
    return wrapper


class WalletError(Exception):
    """A wallet error."""
    pass
