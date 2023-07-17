# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from iota_sdk.types.output_id import OutputId
from dataclasses import dataclass
from typing import Dict, Optional
import humps


class NodeIndexerAPI():

    @dataclass
    class QueryParameters:
        address: Optional[str] = None
        alias_address: Optional[str] = None
        created_after: Optional[int] = None
        created_before: Optional[int] = None
        cursor: Optional[str] = None
        expiration_return_address: Optional[str] = None
        expires_after: Optional[int] = None
        expires_before: Optional[int] = None
        governor: Optional[str] = None
        has_expiration: Optional[bool] = None
        has_native_tokens: Optional[bool] = None
        has_storage_deposit_return: Optional[bool] = None
        has_timelock: Optional[bool] = None
        issuer: Optional[str] = None
        max_native_token_count: Optional[int] = None
        min_native_token_count: Optional[int] = None
        page_size: Optional[int] = None
        sender: Optional[str] = None
        state_controller: Optional[str] = None
        storage_deposit_return_address: Optional[str] = None
        tag: Optional[str] = None
        timelocked_after: Optional[int] = None
        timelocked_before: Optional[int] = None

        def as_dict(self):
            return humps.camelize([{k: v} for k, v in self.__dict__.items() if v != None])

    class OutputIdsResponse:
        def __init__(self, dict: Dict):
            self.ledgerIndex = dict["ledgerIndex"]
            self.cursor = dict["cursor"]
            self.items = [OutputId.from_string(
                output_id) for output_id in dict["items"]]

    def basic_output_ids(self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch basic output IDs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('basicOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return self.OutputIdsResponse(response)

    def alias_output_ids(self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch alias output IDs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('aliasOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return self.OutputIdsResponse(response)

    def alias_output_id(self, alias_id: HexStr) -> OutputId:
        """Fetch alias output ID.
        """
        return OutputId.from_string(self._call_method('aliasOutputId', {
            'aliasId': alias_id
        }))

    def nft_output_ids(self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch NFT output IDs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('nftOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return self.OutputIdsResponse(response)

    def nft_output_id(self, nft_id: HexStr) -> OutputId:
        """Fetch NFT output ID.
        """
        return OutputId.from_string(self._call_method('nftOutputId', {
            'nftId': nft_id
        }))

    def foundry_output_ids(self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch foundry Output IDs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('foundryOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return self.OutputIdsResponse(response)

    def foundry_output_id(self, foundry_id: HexStr) -> OutputId:
        """Fetch foundry Output ID.
        """
        return OutputId.from_string(self._call_method('foundryOutputId', {
            'foundryId': foundry_id
        }))
