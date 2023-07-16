# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from dataclasses import dataclass


@dataclass
class NativeToken():
    """Represents a native token.

    Attributes:
        id (HexStr): The unique identifier of the token
        amount (HexStr): The amount of the token
    """
    id: HexStr
    amount: HexStr
