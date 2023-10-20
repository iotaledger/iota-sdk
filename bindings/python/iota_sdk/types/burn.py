# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations  # Allow reference to Burn in Burn class
from typing import List, Optional, Dict, Any
from dataclasses import dataclass
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class Burn:
    """A DTO for `Burn`.

    Attributes:
        accounts: The accounts to burn.
        nfts: The NFTs to burn.
        foundries: The foundries to burn.
        native_tokens: The native tokens to burn.
    """

    accounts: Optional[List[HexStr]] = None
    nfts: Optional[List[HexStr]] = None
    foundries: Optional[List[HexStr]] = None
    native_tokens: Optional[List[NativeToken]] = None

    def add_account(self, account: HexStr) -> Burn:
        """Add an account to the burn.
        """
        if self.accounts is None:
            self.accounts = []
        self.accounts.append(account)
        return self

    def add_nft(self, nft: HexStr) -> Burn:
        """Add an NFT to the burn.
        """
        if self.nfts is None:
            self.nfts = []
        self.nfts.append(nft)
        return self

    def add_foundry(self, foundry: HexStr) -> Burn:
        """Add a foundry to the burn.
        """
        if self.foundries is None:
            self.foundries = []
        self.foundries.append(foundry)
        return self

    def add_native_token(self, native_token: NativeToken) -> Burn:
        """Add a native token to the burn.
        """
        if self.native_tokens is None:
            self.native_tokens = [native_token]
        else:
            self.native_tokens.append(native_token)
        return self
