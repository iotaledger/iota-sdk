# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import IntEnum
from iota_sdk.types.common import HexStr, json


class ContextInputType(IntEnum):
    """Context input types.
    """
    BlockIssuanceCredit = 1


@json
@dataclass
class BlockIssuanceCreditContextInput:
    """A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
    the BIC vector of a specific slot.
    """
    type: int
    account_id: HexStr
