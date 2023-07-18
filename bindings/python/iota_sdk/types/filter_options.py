# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional


@dataclass
class FilterOptions:
    """Options to filter outputs.
    """

    lowerBoundBookedTimestamp: Optional[int] = None
    upperBoundBookedTimestamp: Optional[int] = None
    outputTypes: Optional[List[int]] = None
    aliasIds: Optional[List[str]] = None
    foundryIds: Optional[List[str]] = None
    nftIds: Optional[List[str]] = None
