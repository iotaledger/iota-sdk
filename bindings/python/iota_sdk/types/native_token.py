# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import hex_str_decoder, HexStr, json


@json
@dataclass
class NativeToken:
    """A native token.

    Attributes:
        id: The unique identifier of the native token.
        amount: The amount of native tokens.
    """
    id: HexStr
    amount: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
