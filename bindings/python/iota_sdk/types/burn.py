# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations  # Allow reference to Burn in Burn class
from dataclasses import dataclass
from typing import List, Optional, Dict, Any
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.common import HexStr


@dataclass
class Burn:
    """A DTO for [`Burn`]

    Parameters:
    -----------
    aliases: Optional[List[str]]
        The aliases (hex encoded) to burn
    nfts: Optional[List[str]]
        The NFTs (hex encoded) to burn
    foundries: Optional[List[str]]
        The foundries (hex encoded) to burn
    nativeTokens: Optional[List[NativeToken]]
        The native tokens to burn
    """

    aliases: Optional[List[HexStr]] = None
    nfts: Optional[List[HexStr]] = None
    foundries: Optional[List[HexStr]] = None
    nativeTokens: Optional[List[NativeToken]] = None

    def add_alias(self, alias: HexStr) -> Burn:
        if self.aliases is None:
            self.aliases = []
        self.aliases.append(alias)
        return self

    def add_nft(self, nft: HexStr) -> Burn:
        if self.nfts is None:
            self.nfts = []
        self.nfts.append(nft)
        return self

    def add_foundry(self, foundry: HexStr) -> Burn:
        if self.foundries is None:
            self.foundries = []
        self.foundries.append(foundry)
        return self

    def add_native_token(self, native_token: NativeToken) -> Burn:
        if self.nativeTokens is None:
            self.nativeTokens = [native_token]
        else:
            self.nativeTokens.append(native_token)
        return self

    def as_dict(self) -> Dict[str, Any]:
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if "nativeTokens" in config:
            config["nativeTokens"] = {nativeToken.__dict__["id"]: nativeToken.__dict__["amount"] for nativeToken in config["nativeTokens"]}
        return config
