# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from typing import Dict, Optional
from abc import ABCMeta, abstractmethod
import humps

from iota_sdk.types.common import HexStr
from iota_sdk.types.output_id import OutputId


class OutputIdsResponse:
    """Response type for output IDs.

    Attributes:
        ledger_index: The ledger index for which the response is valid.
        cursor: The cursor to the next page of results.
        items: The query results.
    """

    def __init__(self, output_dict: Dict):
        self.ledgerIndex = output_dict["ledgerIndex"]
        self.cursor = output_dict["cursor"]
        self.items = [OutputId.from_string(
            output_id) for output_id in output_dict["items"]]


class NodeIndexerAPI(metaclass=ABCMeta):
    """Node indexer API.
    """

    @dataclass
    class QueryParameters:
        """Query parameters

        **Attributes:**
        address :
            Bech32-encoded address that should be searched for.
        alias_address :
            Filter foundry outputs based on bech32-encoded address of the controlling alias.
        created_after :
            Returns outputs that were created after a certain Unix timestamp.
            created_before :
            Returns outputs that were created before a certain Unix timestamp.
            cursor :
            Starts the search from the cursor (confirmationMS+outputId.pageSize).
            expiration_return_address :
            Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
            condition.
            expires_after :
            Returns outputs that expire after a certain Unix timestamp.
            expires_before :
            Returns outputs that expire before a certain Unix timestamp.
            governor :
            Filters outputs based on bech32-encoded governor (governance controller) address.
            has_expiration :
            Filters outputs based on the presence of expiration unlock condition.
            has_native_tokens :
            Filters outputs based on the presence of native tokens.
            has_storage_deposit_return :
            Filters outputs based on the presence of storage deposit return unlock condition.
            has_timelock :
            Filters outputs based on the presence of timelock unlock condition.
            issuer:
            Filters outputs based on bech32-encoded issuer address.
            max_native_token_count :
            Filters outputs that have at most a certain number of distinct native tokens.
            min_native_token_count :
            Filters outputs that have at least a certain number of distinct native tokens.
            page_size :
            The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
            returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
            sender :
            Filters outputs based on the presence of validated Sender (bech32 encoded).
            state_controller :
            Filters outputs based on bech32-encoded state controller address.
            storage_deposit_return_address :
            Filters outputs based on the presence of a specific return address in the storage deposit return unlock
            condition.
            tag :
            Filters outputs based on matching Tag Block.
            timelocked_after :
            Returns outputs that are timelocked after a certain Unix timestamp.
            timelocked_before :
            Returns outputs that are timelocked before a certain Unix timestamp.
         unlockable_by_address :
            Returns outputs that are unlockable by the bech32 address.
        """
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
        unlockable_by_address: Optional[str] = None

        def as_dict(self):
            """Converts this object to a dict.
            """
            return humps.camelize(
                [{k: v} for k, v in self.__dict__.items() if v is not None])

    @abstractmethod
    def _call_method(self, name, data=None):
        """
        Sends a message to the Rust library and returns the response.
        It is abstract here as its implementation is located in `client.py`, which is a composite class.

        Arguments:

        * `name`: The `name` parameter is a string that represents the name of the method to be called.
        It is used to identify the specific method to be executed in the Rust library.
        * `data`: The `data` parameter is an optional parameter that represents additional data to be
        sent along with the method call. It is a dictionary that contains key-value pairs of data. If
        the `data` parameter is provided, it will be included in the `message` dictionary as the 'data'
        key.

        Returns:

        The method returns either the payload from the JSON response or the entire response if there is
        no payload.
        """

    def output_ids(
            self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch alias/basic/NFT/foundry output IDs from the given query parameters.
        Supported query parameters are: "hasNativeTokens", "minNativeTokenCount", "maxNativeTokenCount", "unlockableByAddress", "createdBefore", "createdAfter", "cursor", "pageSize".

        Returns:
            The corresponding output IDs of the outputs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('outputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return OutputIdsResponse(response)

    def basic_output_ids(
            self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch basic output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the basic outputs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('basicOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return OutputIdsResponse(response)

    def alias_output_ids(
            self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch alias output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the alias outputs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('aliasOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return OutputIdsResponse(response)

    def alias_output_id(self, alias_id: HexStr) -> OutputId:
        """Fetch alias output ID from the given alias ID.

        Returns:
            The output ID of the alias output.
        """
        return OutputId.from_string(self._call_method('aliasOutputId', {
            'aliasId': alias_id
        }))

    def nft_output_ids(
            self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch NFT output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the NFT outputs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('nftOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return OutputIdsResponse(response)

    def nft_output_id(self, nft_id: HexStr) -> OutputId:
        """Fetch NFT output ID from the given NFT ID.

        Returns:
            The output ID of the NFT output.
        """
        return OutputId.from_string(self._call_method('nftOutputId', {
            'nftId': nft_id
        }))

    def foundry_output_ids(
            self, query_parameters: QueryParameters) -> OutputIdsResponse:
        """Fetch foundry Output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the foundry outputs.
        """

        query_parameters_camelized = query_parameters.as_dict()

        response = self._call_method('foundryOutputIds', {
            'queryParameters': query_parameters_camelized,
        })
        return OutputIdsResponse(response)

    def foundry_output_id(self, foundry_id: HexStr) -> OutputId:
        """Fetch foundry Output ID from the given foundry ID.

        Returns:
            The output ID of the foundry output.
        """
        return OutputId.from_string(self._call_method('foundryOutputId', {
            'foundryId': foundry_id
        }))
