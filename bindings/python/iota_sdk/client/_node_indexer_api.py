# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from typing import Optional
from abc import ABCMeta, abstractmethod
from iota_sdk.client.responses import OutputIdsResponse
from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.output_id import OutputId


class NodeIndexerAPI(metaclass=ABCMeta):
    """Node indexer API.
    """

    @json
    @dataclass
    class CommonQueryParameters:
        """Common Query parameters

        **Attributes:**

        page_size:
            The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
            returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
        cursor:
            Starts the search from the cursor (createdSlotIndex+outputId.pageSize).
            If an empty string is provided, only the first page is returned.
        created_before:
            Returns outputs that were created before a certain slot index.
        created_after:
            Returns outputs that were created after a certain slot index.
        """
        page_size: Optional[int] = None
        cursor: Optional[str] = None
        created_before: Optional[SlotIndex] = None
        created_after: Optional[SlotIndex] = None

    @json
    @dataclass
    class OutputQueryParameters(CommonQueryParameters):
        """Output Query parameters

        **Attributes:**

        has_native_token:
            Filters outputs based on the presence of a native token.
        native_token:
            Filters outputs based on the presence of a specific native token.
        unlockable_by_address:
            Returns outputs that are unlockable by the bech32 address.
        """
        has_native_token: Optional[bool] = None
        native_token: Optional[HexStr] = None
        unlockable_by_address: Optional[str] = None

    @json
    @dataclass
    class BasicOutputQueryParameters(CommonQueryParameters):
        """Basic Output Query parameters

        **Attributes: **
        has_native_token:
            Filters outputs based on the presence of a native token.
        native_token:
            Filters outputs based on the presence of a specific native token.
        unlockable_by_address:
            Returns outputs that are unlockable by the bech32 address.
        address:
            Bech32-encoded address that should be searched for.
        has_storage_deposit_return:
            Filters outputs based on the presence of storage deposit return unlock condition.
        storage_deposit_return_address:
            Filters outputs based on the presence of a specific return address in the storage deposit return unlock condition.
        has_expiration:
            Filters outputs based on the presence of expiration unlock condition.
        expiration_return_address:
            Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock condition.
        expires_before:
            Returns outputs that expire before a certain slot index.
        expires_after:
            Returns outputs that expire after a certain slot index.
        has_timelock:
            Filters outputs based on the presence of timelock unlock condition.
        timelocked_before:
            Returns outputs that are timelocked before a certain slot index.
        timelocked_after:
            Returns outputs that are timelocked after a certain slot index.
        sender:
            Filters outputs based on the presence of validated Sender (bech32 encoded).
        tag:
            Filters outputs based on matching Tag Block.
        """
        has_native_token: Optional[bool] = None
        native_token: Optional[HexStr] = None
        unlockable_by_address: Optional[str] = None
        address: Optional[str] = None
        has_storage_deposit_return: Optional[bool] = None
        storage_deposit_return_address: Optional[str] = None
        has_expiration: Optional[bool] = None
        expiration_return_address: Optional[str] = None
        expires_before: Optional[SlotIndex] = None
        expires_after: Optional[SlotIndex] = None
        has_timelock: Optional[bool] = None
        timelocked_before: Optional[SlotIndex] = None
        timelocked_after: Optional[SlotIndex] = None
        sender: Optional[str] = None
        tag: Optional[HexStr] = None

    @json
    @dataclass
    class AccountOutputQueryParameters(CommonQueryParameters):
        """Account Output Query parameters

        **Attributes: **
        address:
            Bech32-encoded address that should be searched for.
        issuer:
            Filters outputs based on bech32-encoded issuer address.
        sender:
            Filters outputs based on the presence of validated Sender (bech32 encoded).
        """
        address: Optional[str] = None
        issuer: Optional[str] = None
        sender: Optional[str] = None

    @json
    @dataclass
    class AnchorOutputQueryParameters(CommonQueryParameters):
        """Anchor Output Query parameters

        **Attributes: **
        unlockable_by_address:
            Returns outputs that are unlockable by the bech32 address.
        state_controller:
            Filters outputs based on bech32-encoded state controller address.
        governor:
            Filters outputs based on bech32-encoded governor (governance controller) address.
        issuer:
            Filters outputs based on bech32-encoded issuer address.
        sender:
            Filters outputs based on the presence of validated Sender (bech32 encoded).
        """
        unlockable_by_address: Optional[str] = None
        state_controller: Optional[str] = None
        governor: Optional[str] = None
        issuer: Optional[str] = None
        sender: Optional[str] = None

    @json
    @dataclass
    class DelegationOutputQueryParameters(CommonQueryParameters):
        """Delegation Output Query parameters

        **Attributes: **
        address:
            Bech32-encoded address that should be searched for.
        validator:
            Filter foundry outputs based on bech32-encoded address of the validator.
        """
        address: Optional[str] = None
        validator: Optional[str] = None

    @json
    @dataclass
    class FoundryOutputQueryParameters(CommonQueryParameters):
        """Foundry Output Query parameters

        **Attributes: **
        has_native_token:
            Filters outputs based on the presence of a native token.
        native_token:
            Filters outputs based on the presence of a specific native token.
        account:
            Filter foundry outputs based on bech32-encoded address of the controlling account.
        """
        has_native_token: Optional[bool] = None
        native_token: Optional[HexStr] = None
        account: Optional[str] = None

    @json
    @dataclass
    class NftOutputQueryParameters(CommonQueryParameters):
        """NFT Output Query parameters

        **Attributes: **
        unlockable_by_address:
            Returns outputs that are unlockable by the bech32 address.
        address:
            Bech32-encoded address that should be searched for.
        has_storage_deposit_return:
            Filters outputs based on the presence of storage deposit return unlock condition.
        storage_deposit_return_address:
            Filters outputs based on the presence of a specific return address in the storage deposit return unlock condition.
        has_expiration:
            Filters outputs based on the presence of expiration unlock condition.
        expiration_return_address:
            Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock condition.
        expires_before:
            Returns outputs that expire before a certain slot index.
        expires_after:
            Returns outputs that expire after a certain slot index.
        has_timelock:
            Filters outputs based on the presence of timelock unlock condition.
        timelocked_before:
            Returns outputs that are timelocked before a certain slot index.
        timelocked_after:
            Returns outputs that are timelocked after a certain slot index.
        issuer:
            Filters outputs based on bech32-encoded issuer address.
        sender:
            Filters outputs based on the presence of validated Sender (bech32 encoded).
        tag:
            Filters outputs based on matching Tag Block.
        """
        unlockable_by_address: Optional[str] = None
        address: Optional[str] = None
        has_storage_deposit_return: Optional[bool] = None
        storage_deposit_return_address: Optional[str] = None
        has_expiration: Optional[bool] = None
        expiration_return_address: Optional[str] = None
        expires_before: Optional[SlotIndex] = None
        expires_after: Optional[SlotIndex] = None
        has_timelock: Optional[bool] = None
        timelocked_before: Optional[SlotIndex] = None
        timelocked_after: Optional[SlotIndex] = None
        issuer: Optional[str] = None
        sender: Optional[str] = None
        tag: Optional[HexStr] = None

    @abstractmethod
    def _call_method(self, name, data=None):
        return {}

    def output_ids(
            self, query_parameters: OutputQueryParameters) -> OutputIdsResponse:
        """Fetch account/anchor/basic/delegation/NFT/foundry output IDs from the given query parameters.
        Supported query parameters are: "hasNativeTokens", "minNativeTokenCount", "maxNativeTokenCount", "unlockableByAddress", "createdBefore", "createdAfter", "cursor", "pageSize".

        Returns:
            The corresponding output IDs of the outputs.
        """

        response = self._call_method('outputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def basic_output_ids(
            self, query_parameters: BasicOutputQueryParameters) -> OutputIdsResponse:
        """Fetch basic output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the basic outputs.
        """

        response = self._call_method('basicOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def account_output_ids(
            self, query_parameters: AccountOutputQueryParameters) -> OutputIdsResponse:
        """Fetch account output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the account outputs.
        """

        response = self._call_method('accountOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def account_output_id(self, account_id: HexStr) -> OutputId:
        """Fetch account output ID from the given account ID.

        Returns:
            The output ID of the account output.
        """
        return OutputId(self._call_method('accountOutputId', {
            'accountId': account_id
        }))

    def anchor_output_ids(
            self, query_parameters: AnchorOutputQueryParameters) -> OutputIdsResponse:
        """Fetch anchor output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the anchor outputs.
        """

        response = self._call_method('anchorOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def anchor_output_id(self, anchor_id: HexStr) -> OutputId:
        """Fetch anchor output ID from the given anchor ID.

        Returns:
            The output ID of the anchor output.
        """
        return OutputId(self._call_method('anchorOutputId', {
            'anchorId': anchor_id
        }))

    def delegation_output_ids(
            self, query_parameters: DelegationOutputQueryParameters) -> OutputIdsResponse:
        """Fetch delegation output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the delegation outputs.
        """

        response = self._call_method('delegationOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def delegation_output_id(self, delegation_id: HexStr) -> OutputId:
        """Fetch delegation output ID from the given delegation ID.

        Returns:
            The output ID of the delegation output.
        """
        return OutputId(self._call_method('delegationOutputId', {
            'delegationId': delegation_id
        }))

    def foundry_output_ids(
            self, query_parameters: FoundryOutputQueryParameters) -> OutputIdsResponse:
        """Fetch foundry Output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the foundry outputs.
        """

        response = self._call_method('foundryOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def foundry_output_id(self, foundry_id: HexStr) -> OutputId:
        """Fetch foundry Output ID from the given foundry ID.

        Returns:
            The output ID of the foundry output.
        """
        return OutputId(self._call_method('foundryOutputId', {
            'foundryId': foundry_id
        }))

    def nft_output_ids(
            self, query_parameters: NftOutputQueryParameters) -> OutputIdsResponse:
        """Fetch NFT output IDs from the given query parameters.

        Returns:
            The corresponding output IDs of the NFT outputs.
        """

        response = self._call_method('nftOutputIds', {
            'queryParameters': query_parameters,
        })
        return OutputIdsResponse(response)

    def nft_output_id(self, nft_id: HexStr) -> OutputId:
        """Fetch NFT output ID from the given NFT ID.

        Returns:
            The output ID of the NFT output.
        """
        return OutputId(self._call_method('nftOutputId', {
            'nftId': nft_id
        }))
