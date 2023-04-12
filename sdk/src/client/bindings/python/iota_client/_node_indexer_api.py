from iota_client._base_api import BaseAPI
from dataclasses import dataclass
import humps


class NodeIndexerAPI(BaseAPI):

    def basic_output_ids(self, query_parameters):
        """Fetch basic output IDs.
        """

        query_parameters = humps.camelize(query_parameters.as_dict())

        return self.send_message('basicOutputIds', {
            'queryParameters': query_parameters,
        })

    def alias_output_ids(self, query_parameters):
        """Fetch alias output IDs.
        """

        query_parameters = humps.camelize(query_parameters.as_dict())

        return self.send_message('aliasOutputIds', {
            'queryParameters': query_parameters,
        })

    def alias_output_id(self, alias_id):
        """Fetch alias output ID.
        """
        return self.send_message('aliasOutputId', {
            'aliasId': alias_id
        })

    def nft_output_ids(self, query_parameters):
        """Fetch NFT output IDs.
        """

        query_parameters = humps.camelize(query_parameters.as_dict())

        return self.send_message('nftOutputIds', {
            'queryParameters': query_parameters,
        })

    def nft_output_id(self, nft_id):
        """Fetch NFT output ID.
        """
        return self.send_message('nftOutputId', {
            'nftId': nft_id
        })

    def foundry_output_ids(self, query_parameters):
        """Fetch foundry Output IDs.
        """

        query_parameters = humps.camelize(query_parameters.as_dict())

        return self.send_message('foundryOutputIds', {
            'queryParameters': query_parameters,
        })

    def foundry_output_id(self, foundry_id):
        """Fetch foundry Output ID.
        """
        return self.send_message('foundryOutputId', {
            'foundryId': foundry_id
        })

    @dataclass
    class QueryParameter:
        address: str = None
        alias_address: str = None
        created_after: int = None
        created_before: int = None
        cursor: str = None
        expiration_return_address: str = None
        expires_after: int = None
        expires_before: int = None
        governor: str = None
        has_expiration: bool = None
        has_native_tokens: bool = None
        has_storage_deposit_return: bool = None
        has_timelock: bool = None
        issuer: str = None
        max_native_token_count: int = None
        min_native_token_count: int = None
        page_size: int = None
        sender: str = None
        state_controller: str = None
        storage_deposit_return_address: str = None
        tag: str = None
        timelocked_after: int = None
        timelocked_before: int = None

        def as_dict(self):
            return [{k: v} for k, v in self.__dict__.items() if v != None]
