# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from dataclasses import dataclass


@dataclass
class NativeToken():
    id: HexStr
    amount: HexStr
