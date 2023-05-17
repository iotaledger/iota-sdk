# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations  # For Burn reference in Burn
from dataclasses import dataclass
from typing import List, Optional, Dict, Any
from iota_sdk.types.native_token import NativeToken


@dataclass
class Burn:
    """A DTO for [`Burn`]

    Parameters:
    -----------
    aliases: Optional[List[str]]
        The aliases to burn (hex encoded)
    nfts: Optional[List[str]]
        The NFTs to burn (hex encoded)
    foundries: Optional[List[str]]
        The foundries to burn (hex encoded)
    nativeTokens: Optional[List[NativeToken]]
        The native tokens of the burn
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
