# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import HexStr


@dataclass
class NativeToken():
    """A native token.

    Attributes:
        id: The unique identifier of the native token.
        amount: The amount of native tokens.
    """
    id: HexStr
    amount: HexStr
