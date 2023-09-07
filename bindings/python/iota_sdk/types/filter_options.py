# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import List, Optional

from dataclasses import dataclass

from iota_sdk.types.common import json


@json
@dataclass
class FilterOptions:
    """Options to filter outputs.
    """

    lowerBoundBookedTimestamp: Optional[int] = None
    upperBoundBookedTimestamp: Optional[int] = None
    outputTypes: Optional[List[int]] = None
    accountIds: Optional[List[str]] = None
    foundryIds: Optional[List[str]] = None
    nftIds: Optional[List[str]] = None
