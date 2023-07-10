# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import HexStr

@dataclass
class Ed25519Signature():
    publicKey: HexStr
    signature: HexStr
    type: int = 0
