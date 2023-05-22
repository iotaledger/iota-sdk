# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations  # Allow reference to Burn in Burn class
from dataclasses import dataclass
from typing import List, Optional, Dict, Any
from iota_sdk.types.native_token import NativeToken


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

    aliases: Optional[List[str]]
    nfts: Optional[List[str]]
    foundries: Optional[List[str]]
    nativeTokens: Optional[List[NativeToken]]

    def add_alias(self, alias: str) -> Burn:
        if self.aliases is None:
            self.aliases = []
        self.aliases.append(alias)
        return self

    def add_nft(self, nft: str) -> Burn:
        if self.nfts is None:
            self.nfts = []
        self.nfts.append(nft)
        return self

    def add_foundry(self, foundry: str) -> Burn:
        if self.foundries is None:
            self.foundries = []
        self.foundries.append(foundry)
        return self

    def add_native_token(self, native_token: NativeToken) -> Burn:
        if self.native_tokens is None:
            self.native_tokens = []
        self.nativeTokens.append(native_token)
        return self

    def as_dict(self) -> Dict[str, Any]:
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if "nativeTokens" in config:
            config["nativeTokens"] = [
                native_token.as_dict() for native_token in config["nativeTokens"]
            ]
        return config
