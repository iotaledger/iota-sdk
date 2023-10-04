# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from json import dumps, loads
from datetime import timedelta
from typing import Any, Dict, List, Optional, Union
import humps
from dacite import from_dict

from iota_sdk.external import create_client, call_client_method, listen_mqtt
from iota_sdk.client._node_core_api import NodeCoreAPI
from iota_sdk.client._node_indexer_api import NodeIndexerAPI
from iota_sdk.client._high_level_api import HighLevelAPI
from iota_sdk.client._utils import ClientUtils
from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager
from iota_sdk.types.block.wrapper import BlockWrapper
from iota_sdk.types.common import HexStr, Node
from iota_sdk.types.feature import BaseFeature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.network_info import NetworkInfo
from iota_sdk.types.output import AccountOutput, BasicOutput, FoundryOutput, NftOutput, deserialize_output
from iota_sdk.types.payload import BasePayload, TransactionPayload
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import BaseUnlockCondition
from iota_sdk.types.transaction_data import PreparedTransactionData


class ClientError(Exception):
    """Represents a client error."""


class Client(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, ClientUtils):
    """Represents an IOTA client.

    Attributes:
        handle: The handle to the inner client object.
    """

    # pylint: disable=unused-argument
    def __init__(
        self,
        nodes: Optional[Union[str, List[str]]] = None,
        primary_node: Optional[str] = None,
        permanode: Optional[str] = None,
        ignore_node_health: Optional[bool] = None,
        api_timeout: Optional[timedelta] = None,
        node_sync_interval: Optional[timedelta] = None,
        quorum: Optional[bool] = None,
        min_quorum_size: Optional[int] = None,
        quorum_threshold: Optional[int] = None,
        user_agent: Optional[str] = None,
        client_handle=None
    ):
        """Initialize the IOTA Client.

        **Arguments**
        nodes :
            A single Node URL or an array of URLs.
        primary_node :
            Node which will be tried first for all requests.
        permanode :
            Permanode URL.
        ignore_node_health :
            If the node health should be ignored.
        api_timeout :
            Timeout for API requests.
        node_sync_interval :
            Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
        quorum :
            If node quorum is enabled. Will compare the responses from multiple nodes and only returns the response if 'quorum_threshold'% of the nodes return the same one.
        min_quorum_size :
            Minimum amount of nodes required for request when quorum is enabled.
        quorum_threshold :
            % of nodes that have to return the same response so it gets accepted.
        user_agent :
            The User-Agent header for requests.
        client_handle :
            An instance of a node client.
        """
        client_config = dict(locals())
        del client_config['self']
        # Delete client_handle, because it's not needed here and can't be
        # serialized.
        if "client_handle" in client_config:
            del client_config["client_handle"]

        if isinstance(nodes, list):
            nodes = [node.to_dict() if isinstance(node, Node)
                     else node for node in nodes]
        elif nodes:
            if isinstance(nodes, Node):
                nodes = [nodes.to_dict()]
            else:
                nodes = [nodes]
        client_config['nodes'] = nodes

        client_config = {
            k: v for k,
            v in client_config.items() if v is not None}

        def get_remaining_nano_seconds(duration: timedelta):
            return (int(duration / timedelta(microseconds=1)) -
                    int(duration.total_seconds()) * 1_000_000) * 1_000

        if 'api_timeout' in client_config:
            client_config['api_timeout'] = {'secs': int(client_config['api_timeout'].total_seconds(
            )), 'nanos': get_remaining_nano_seconds(client_config['api_timeout'])}
        if 'node_sync_interval' in client_config:
            client_config['node_sync_interval'] = {'secs': int(client_config['node_sync_interval'].total_seconds(
            )), 'nanos': get_remaining_nano_seconds(client_config['node_sync_interval'])}

        client_config = humps.camelize(client_config)
        client_config_str = dumps(client_config)

        # Create the message handler
        if client_handle is None:
            self.handle = create_client(client_config_str)
        else:
            self.handle = client_handle

    def _call_method(self, name, data=None):
        """Dumps json string and calls `call_client_method()`
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
        return response

    def get_handle(self):
        """Get the client handle.

        Returns:
            The inner client object.
        """
        return self.handle

    def build_account_output(self,
                             account_id: HexStr,
                             unlock_conditions: List[BaseUnlockCondition],
                             amount: Optional[int] = None,
                             mana: Optional[int] = None,
                             native_tokens: Optional[List[NativeToken]] = None,
                             state_index: Optional[int] = None,
                             state_metadata: Optional[str] = None,
                             foundry_counter: Optional[int] = None,
                             features: Optional[List[BaseFeature]] = None,
                             immutable_features: Optional[List[BaseFeature]] = None) -> AccountOutput:
        """Build an AccountOutput.

        Args:
            account_id: A unique ID for the new account.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            native_tokens: Native tokens added to the new output.
            state_index: A counter that must increase by 1 every time the account is state transitioned.
            state_metadata: Metadata that can only be changed by the state controller.
            foundry_counter: A counter that denotes the number of foundries created by this account output.
            features: A list of features.
            immutable_features: A list of immutable features.

        Returns:
            The account output as dict.
        """

        unlock_conditions = [unlock_condition.to_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.to_dict()
                             for native_token in native_tokens]

        if features:
            features = [feature.to_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.to_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        if mana:
            mana = str(mana)

        return deserialize_output(self._call_method('buildAccountOutput', {
            'accountId': account_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'mana': mana,
            'nativeTokens': native_tokens,
            'stateIndex': state_index,
            'stateMetadata': state_metadata,
            'foundryCounter': foundry_counter,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_basic_output(self,
                           unlock_conditions: List[BaseUnlockCondition],
                           amount: Optional[int] = None,
                           mana: Optional[int] = None,
                           native_tokens: Optional[List[NativeToken]] = None,
                           features: Optional[List[BaseFeature]] = None) -> BasicOutput:
        """Build a BasicOutput.

        Args:
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            native_tokens: Native tokens added to the new output.
            features: Features that add utility to the output but do not impose unlocking conditions.

        Returns:
            The basic output as dict.
        """

        unlock_conditions = [unlock_condition.to_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.to_dict()
                             for native_token in native_tokens]

        if features:
            features = [feature.to_dict() for feature in features]

        if amount:
            amount = str(amount)

        if mana:
            mana = str(mana)

        return deserialize_output(self._call_method('buildBasicOutput', {
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'mana': mana,
            'nativeTokens': native_tokens,
            'features': features,
        }))

    def build_foundry_output(self,
                             serial_number: int,
                             token_scheme: SimpleTokenScheme,
                             unlock_conditions: List[BaseUnlockCondition],
                             amount: Optional[int] = None,
                             native_tokens: Optional[List[NativeToken]] = None,
                             features: Optional[List[BaseFeature]] = None,
                             immutable_features: Optional[List[BaseFeature]] = None) -> FoundryOutput:
        """Build a FoundryOutput.

        Args:
            serial_number: The serial number of the foundry with respect to the controlling account.
            token_scheme: Defines the supply control scheme of the tokens controlled by the foundry. Currently only a simple scheme is supported.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            native_tokens: Native tokens added to the new output.
            features: Features that add utility to the output but do not impose unlocking conditions.
            immutable_features: Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.

        Returns:
            The foundry output as dict.
        """

        unlock_conditions = [unlock_condition.to_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.__dict__
                             for native_token in native_tokens]

        if features:
            features = [feature.to_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.to_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        return deserialize_output(self._call_method('buildFoundryOutput', {
            'serialNumber': serial_number,
            'tokenScheme': token_scheme.to_dict(),
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_nft_output(self,
                         nft_id: HexStr,
                         unlock_conditions: List[BaseUnlockCondition],
                         amount: Optional[int] = None,
                         mana: Optional[int] = None,
                         native_tokens: Optional[List[NativeToken]] = None,
                         features: Optional[List[BaseFeature]] = None,
                         immutable_features: Optional[List[BaseFeature]] = None) -> NftOutput:
        """Build an NftOutput.

        Args:
            nft_id: A unique ID for the new NFT.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            native_tokens: Native tokens added to the new output.
            features: Features that add utility to the output but do not impose unlocking conditions.
            immutable_features: Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.

        Returns:
            The NFT output as dict.
        """

        unlock_conditions = [unlock_condition.to_dict()
                             for unlock_condition in unlock_conditions]

        if native_tokens:
            native_tokens = [native_token.__dict__
                             for native_token in native_tokens]

        if features:
            features = [feature.to_dict() for feature in features]
        if immutable_features:
            immutable_features = [immutable_feature.to_dict()
                                  for immutable_feature in immutable_features]

        if amount:
            amount = str(amount)

        if mana:
            mana = str(mana)

        return deserialize_output(self._call_method('buildNftOutput', {
            'nftId': nft_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'mana': mana,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def get_node(self) -> Dict[str, Any]:
        """Get a node candidate from the healthy node pool.
        """
        return self._call_method('getNode')

    def get_network_info(self) -> NetworkInfo:
        """Gets the network related information such as network_id.
        """
        return NetworkInfo.from_dict(self._call_method('getNetworkInfo'))

    def get_network_id(self) -> int:
        """Gets the network id of the node we're connecting to.
        """
        return int(self._call_method('getNetworkId'))

    def get_bech32_hrp(self) -> str:
        """Returns the bech32_hrp.
        """
        return self._call_method('getBech32Hrp')

    def unhealthy_nodes(self) -> List[Dict[str, Any]]:
        """Returns the unhealthy nodes.
        """
        return self._call_method('unhealthyNodes')

    def sign_transaction(
            self,
            secret_manager: Union[LedgerNanoSecretManager, MnemonicSecretManager, SeedSecretManager, StrongholdSecretManager],
            prepared_transaction_data: PreparedTransactionData) -> TransactionPayload:
        """Sign a transaction.

        Args:
            secret_manager: One of the supported secret managers.
            prepared_transaction_data: a prepared transaction to sign.
        """
        return from_dict(TransactionPayload, self._call_method('signTransaction', {
            'secretManager': secret_manager,
            'preparedTransactionData': prepared_transaction_data
        }))

    def submit_payload(
            self, payload: BasePayload) -> List[Union[HexStr, BlockWrapper]]:
        """Submit a payload in a block.

        Args:
            payload : The payload to submit.

        Returns:
            List of HexStr or Block.
        """
        result = self._call_method('postBlockPayload', {
            'payload': payload.to_dict()
        })
        result[1] = BlockWrapper.from_dict(result[1])
        return result

    def listen_mqtt(self, topics: List[str], handler):
        """Listen to MQTT events.

        Args:
            topics: The topics to listen to.
            handler: A callback function for MQTT events.
        """
        listen_mqtt(self.handle, topics, handler)

    def clear_mqtt_listeners(self, topics: List[str]):
        """Removes all listeners for the provided MQTT topics.

        Args:
            topics: The topics to stop listening to.
        """
        return self._call_method('clearListeners', {
            'topics': topics
        })
