# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr

class Ed25519Signature():
    def __init__(self, public_key: HexStr, signature: HexStr):
        """Initialize an Ed25519Signature"""
        
        self.public_key = public_key
        self.signature = signature

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        return config
