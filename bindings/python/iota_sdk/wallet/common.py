# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from json import dumps, JSONEncoder
from enum import Enum
import humps
from iota_sdk import call_wallet_method


def _call_method_routine(func):
    """The routine of dump json string and call call_wallet_method().
    """
    def wrapper(*args, **kwargs):
        class MyEncoder(JSONEncoder):
            """Custom encoder
            """

            # pylint: disable=too-many-return-statements
            def default(self, o):
                to_dict_method = getattr(o, "to_dict", None)
                if callable(to_dict_method):
                    return o.to_dict()
                as_dict_method = getattr(o, "as_dict", None)
                if callable(as_dict_method):
                    return o.as_dict()
                if isinstance(o, str):
                    return o
                if isinstance(o, Enum):
                    return o.__dict__
                if isinstance(o, dict):
                    return o
                if hasattr(o, "__dict__"):
                    obj_dict = o.__dict__

                    items_method = getattr(self, "items", None)
                    if callable(items_method):
                        for k, v in obj_dict.items():
                            obj_dict[k] = dumps(v, cls=MyEncoder)
                            return obj_dict
                    return obj_dict
                return o
        message = func(*args, **kwargs)

        for k, v in message.items():
            if not isinstance(v, str):
                message[k] = json.loads(dumps(v, cls=MyEncoder))

        def remove_none(obj):
            if isinstance(obj, (list, tuple, set)):
                return type(obj)(remove_none(x) for x in obj if x is not None)
            if isinstance(obj, dict):
                return type(obj)((remove_none(k), remove_none(v))
                                 for k, v in obj.items() if k is not None and v is not None)
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
        return response
    return wrapper


class WalletError(Exception):
    """A wallet error."""
