# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import iota_sdk
from iota_sdk import call_client_method, listen_mqtt
from iota_sdk.client._node_core_api import NodeCoreAPI
from iota_sdk.client._node_indexer_api import NodeIndexerAPI
from iota_sdk.client._high_level_api import HighLevelAPI
from iota_sdk.client._utils import ClientUtils
from iota_sdk.types.block import Block
from iota_sdk.types.common import HexStr, Node
from iota_sdk.types.feature import Feature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output import Output
from iota_sdk.types.token_scheme import TokenScheme
from iota_sdk.types.unlock_condition import UnlockCondition
from json import dumps, loads
import humps
from datetime import timedelta
from typing import Any, Dict, List, Optional
from dacite import from_dict


class ClientError(Exception):
    """client error"""
    pass


class Client(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, ClientUtils):
    def __init__(
        self,
        nodes: Optional[str | List[str]] = None,
        primary_node: Optional[str] = None,
        primary_pow_node: Optional[str] = None,
        permanode: Optional[str] = None,
        ignore_node_health: Optional[bool] = None,
        api_timeout: Optional[timedelta] = None,
        node_sync_interval: Optional[timedelta] = None,
        remote_pow_timeout: Optional[timedelta] = None,
        tips_interval: Optional[int] = None,
        quorum: Optional[bool] = None,
        min_quorum_size: Optional[int] = None,
        quorum_threshold: Optional[int] = None,
        user_agent: Optional[str] = None,
        local_pow: Optional[bool] = None,
        fallback_to_local_pow: Optional[bool] = None,
        pow_worker_count: Optional[int] = None,
        client_handle=None
    ):
        """Initialize the IOTA Client.

        Parameters
        ----------
        nodes : string or array of string
            A single Node URL or an array of URLs.
        primary_node : string
            Node which will be tried first for all requests.
        primary_pow_node : string
            Node which will be tried first when using remote PoW, even before the primary_node.
        permanode : string
            Permanode URL.
        ignore_node_health : bool
            If the node health should be ignored.
        api_timeout : datetime.timedelta
            Timeout for API requests.
        node_sync_interval : datetime.timedelta
            Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
        remote_pow_timeout : datetime.timedelta
            Timeout when sending a block that requires remote proof of work.
        tips_interval : int
            Tips request interval during PoW in seconds.
        quorum : bool
            If node quorum is enabled. Will compare the responses from multiple nodes 
            and only returns the response if `quorum_threshold`% of the nodes return the same one.
        min_quorum_size : int
            Minimum amount of nodes required for request when quorum is enabled.
        quorum_threshold : int
            % of nodes that have to return the same response so it gets accepted.
        user_agent : string
            The User-Agent header for requests.
        local_pow : bool
            Local proof of work.
        fallback_to_local_pow : bool
            Fallback to local proof of work if the node doesn't support remote PoW.
        pow_worker_count : int
            The amount of threads to be used for proof of work.
        """
        client_config = dict(locals())
        del client_config['self']
        # Delete client_handle, because it's not needed here and can't be serialized
        if "client_handle" in client_config:
            del client_config["client_handle"]

        if isinstance(nodes, list):
            nodes = [node.as_dict() if isinstance(node, Node)
                     else node for node in nodes]
        elif nodes:
            if isinstance(nodes, Node):
                nodes = [nodes.as_dict()]
            else:
                nodes = [nodes]
        client_config['nodes'] = nodes

        client_config = {k: v for k, v in client_config.items() if v != None}

        def get_remaining_nano_seconds(duration: timedelta):
            return (int(duration/timedelta(microseconds=1))-int(duration.total_seconds())*1_000_000)*1_000

        if 'api_timeout' in client_config:
            client_config['api_timeout'] = {'secs': int(client_config['api_timeout'].total_seconds(
            )), 'nanos': get_remaining_nano_seconds(client_config['api_timeout'])}
        if 'node_sync_interval' in client_config:
            client_config['node_sync_interval'] = {'secs': int(client_config['node_sync_interval'].total_seconds(
            )), 'nanos': get_remaining_nano_seconds(client_config['node_sync_interval'])}
        if 'remote_pow_timeout' in client_config:
            client_config['remote_pow_timeout'] = {'secs': int(client_config['remote_pow_timeout'].total_seconds(
            )), 'nanos': get_remaining_nano_seconds(client_config['remote_pow_timeout'])}

        client_config = humps.camelize(client_config)
        client_config_str = dumps(client_config)

        # Create the message handler
        if client_handle is None:
            self.handle = iota_sdk.create_client(client_config_str)
        else:
            self.handle = client_handle

    def _call_method(self, name, data=None):
        """Dumps json string and call call_client_method()
        """
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        message = dumps(message)

        # Send message to the Rust library
        response = call_client_method(self.handle, message)

        json_response = loads(response)

        if "type" in json_response:
            if json_response["type"] == "error":
                raise ClientError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        else:
            return response

    def get_handle(self):
        return self.handle

    def build_alias_output(self,
                           alias_id: HexStr,
                           unlock_conditions: List[UnlockCondition],
                           amount: Optional[int] = None,
                           native_tokens: Optional[List[NativeToken]] = None,
                           state_index: Optional[int] = None,
                           state_metadata: Optional[str] = None,
                           foundry_counter: Optional[int] = None,
                           features: Optional[List[Feature]] = None,
                           immutable_features: Optional[List[Feature]] = None):
        """Build an AliasOutput.

        Parameters
        ----------
        alias_id : string
            Hex encoded alias id
        unlock_conditions : array of UnlockCondition
            The unlock conditions for this output
        amount : int
            Amount of base token
        native_tokens : array of NativeToken
            The native token to add to the output
        state_index : int
            Index of the state
        state_metadata : string
            Hex encoded state metadata
        foundry_counter : int
            Counter of foundry outputs created
        features : array of Feature
            Features for this outputs
        immutable_features : array of Feature
            Immutable features

        Returns
        -------
        Output as dict
        """

        unlock_conditions = [unlock_condition.as_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.as_dict()
                             for native_token in native_tokens]

        if features:
            features = [feature.as_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.as_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        return from_dict(Output, self._call_method('buildAliasOutput', {
            'aliasId': alias_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'stateIndex': state_index,
            'stateMetadata': state_metadata,
            'foundryCounter': foundry_counter,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_basic_output(self,
                           unlock_conditions: List[UnlockCondition],
                           amount: Optional[int] = None,
                           native_tokens: Optional[List[NativeToken]] = None,
                           features: Optional[List[Feature]] = None):
        """Build a BasicOutput.

        Parameters
        ----------
        unlock_conditions : array of UnlockCondition
            The unlock conditions for this output
        amount : int
            Amount of base token
        native_tokens : array of NativeToken
            The native token to add to the output
        features : array of Feature
            Features for this outputs

        Returns
        -------
        Output as dict
        """

        unlock_conditions = [unlock_condition.as_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.as_dict()
                             for native_token in native_tokens]

        if features:
            features = [feature.as_dict() for feature in features]

        if amount:
            amount = str(amount)

        return from_dict(Output, self._call_method('buildBasicOutput', {
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
        }))

    def build_foundry_output(self,
                             serial_number: int,
                             token_scheme: TokenScheme,
                             unlock_conditions: List[UnlockCondition],
                             amount: Optional[int] = None,
                             native_tokens: Optional[List[NativeToken]] = None,
                             features: Optional[List[Feature]] = None,
                             immutable_features: Optional[List[Feature]] = None):
        """Build a FoundryOutput.

        Parameters
        ----------
        serial_number : int
            The serial number of the foundry
        token_scheme : TokenScheme
            The Token scheme. Currently only a simple scheme is supported
        unlock_conditions : array of UnlockCondition
            The unlock conditions for this output
        amount : int
            Amount of base token
        native_tokens : array of NativeToken
            The native token to add to the output
        features : array of Feature
            Features for this outputs
        immutable_features : array of Feature
            Immutable features

        Returns
        -------
        Output as dict
        """

        unlock_conditions = [unlock_condition.as_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.__dict__
                             for native_token in native_tokens]

        if features:
            features = [feature.as_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.as_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        return from_dict(Output, self._call_method('buildFoundryOutput', {
            'serialNumber': serial_number,
            'tokenScheme': token_scheme.as_dict(),
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_nft_output(self,
                         nft_id: HexStr,
                         unlock_conditions: List[UnlockCondition],
                         amount: Optional[int] = None,
                         native_tokens: Optional[List[NativeToken]] = None,
                         features: Optional[List[Feature]] = None,
                         immutable_features: Optional[List[Feature]] = None):
        """Build an NftOutput.

        Parameters
        ----------
        nft_id : string
            Hex encoded nft id
        unlock_conditions : array of UnlockCondition
            The unlock conditions for this output
        amount : int
            Amount of base token
        native_tokens : array of NativeToken
            The native tokens to add to the output
        features : array of Feature
            Features for this outputs
        immutable_features : array of Feature
            Immutable features

        Returns
        -------
        Output as dict
        """

        unlock_conditions = [unlock_condition.as_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.as_dict()
                             for native_token in native_tokens]

        if features:
            features = [feature.as_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.as_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        return from_dict(Output, self._call_method('buildNftOutput', {
            'nftId': nft_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_and_post_block(self,
                             secret_manager=None,
                             account_index: Optional[int] = None,
                             coin_type: Optional[int] = None,
                             custom_remainder_address: Optional[str] = None,
                             data: Optional[HexStr] = None,
                             initial_address_index: Optional[int] = None,
                             input_range_start: Optional[int] = None,
                             input_range_end: Optional[int] = None,
                             inputs: Optional[List[Dict[str, Any]]] = None,
                             output: Optional[Dict[str, Any]] = None,
                             outputs: Optional[List[Any]] = None,
                             tag: Optional[HexStr] = None) -> List[HexStr | Block]:
        """Build and post a block.

        Parameters
        ----------
        account_index : int
            Account Index
        coin_type : int
            Coin type. The CoinType enum can be used
        custom_remainder_address : string
            Address to send the remainder funds to
        data : str
            Hex encoded data
        initial_address_index : int
            Initial address index
        input_range_start : int
            Start of the input range
        input_range_end : int
            End of the input range
        inputs : Array of Inputs
            Inputs to use
        output : Any # TODO: https://github.com/iotaledger/iota-sdk/issues/129
            Address and amount to send to
        outputs : Array of Outputs
            Outputs to use
        tag : string
            Hex encoded tag

        Returns
        -------
        Block as dict

        """
        options = dict(locals())

        del options['self']
        del options['secret_manager']

        options = {k: v for k, v in options.items() if v != None}

        if 'output' in options:
            options['output'] = options.pop('output').as_dict()

        if 'coin_type' in options:
            options['coin_type'] = int(options.pop('coin_type'))

        is_start_set = 'input_range_start' in options
        is_end_set = 'input_range_end' in options
        if is_start_set or is_end_set:
            options['range'] = {}
            if is_start_set:
                options['range']['start'] = options.pop('start')
            if is_end_set:
                options['range']['end'] = options.pop('end')

        options = humps.camelize(options)

        result = self._call_method('buildAndPostBlock', {
            'secretManager': secret_manager,
            'options': options
        })
        result[1] = Block.from_dict(result[1])
        return result

    def get_node(self) -> Dict[str, Any]:
        """Get a node candidate from the healthy node pool.
        """
        return self._call_method('getNode')

    def get_network_info(self) -> Dict[str, Any]:
        """Gets the network related information such as network_id and min_pow_score.
        """
        return self._call_method('getNetworkInfo')

    def get_network_id(self) -> int:
        """Gets the network id of the node we're connecting to.
        """
        return int(self._call_method('getNetworkId'))

    def get_bech32_hrp(self) -> str:
        """Returns the bech32_hrp.
        """
        return self._call_method('getBech32Hrp')

    def get_min_pow_score(self) -> int:
        """Returns the min pow score.
        """
        return int(self._call_method('getMinPowScore'))

    def get_tips_interval(self) -> int:
        """Returns the tips interval.
        """
        return int(self._call_method('getTipsInterval'))

    def get_local_pow(self) -> bool:
        """Returns if local pow should be used or not.
        """
        return self._call_method('getLocalPow')

    def get_fallback_to_local_pow(self) -> bool:
        """Get fallback to local proof of work timeout.
        """
        return self._call_method('getFallbackToLocalPow')

    def unhealthy_nodes(self) -> List[Dict[str, Any]]:
        """Returns the unhealthy nodes.
        """
        return self._call_method('unhealthyNodes')

    def prepare_transaction(self, secret_manager=None, options=None):
        """Prepare a transaction for signing.
        """
        return self._call_method('prepareTransaction', {
            'secretManager': secret_manager,
            'options': options
        })

    def sign_transaction(self, secret_manager, prepared_transaction_data):
        """Sign a transaction.
        """
        return self._call_method('signTransaction', {
            'secretManager': secret_manager,
            'preparedTransactionData': prepared_transaction_data
        })

    def submit_payload(self, payload) -> List[HexStr | Block]:
        """Submit a payload in a block.
        """
        result = self._call_method('postBlockPayload', {
            'payload': payload
        })
        result[1] = Block.from_dict(result[1])
        return result

    def listen_mqtt(self, topics: List[str], handler):
        """Listen to MQTT events.
        """
        listen_mqtt(self.handle, topics, handler)

    def clear_mqtt_listeners(self, topics: List[str]):
        """Removes all listeners for the provided MQTT topics.
        """
        return self._call_method('clearListeners', {
            'topics': topics
        })
