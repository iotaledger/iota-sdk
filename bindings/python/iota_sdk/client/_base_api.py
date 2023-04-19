# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.client.common import call_method_routine


class BaseAPI():

    @call_method_routine
    def call_method(self, name, data=None):
        message = {
            'name': name,
            'data': data
        }

        return message
