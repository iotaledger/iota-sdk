# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from json import dumps
from datetime import timedelta
from typing import Any, Dict, List, Optional, Union
import humps
from iota_sdk.external import create_client, listen_mqtt
from iota_sdk.client._node_core_api import NodeCoreAPI
from iota_sdk.client._node_indexer_api import NodeIndexerAPI
from iota_sdk.client._high_level_api import HighLevelAPI
from iota_sdk.client._utils import ClientUtils
from iota_sdk.client.common import _call_client_method_routine
from iota_sdk.types.block.block import UnsignedBlock
from iota_sdk.types.client_options import MqttBrokerOptions
from iota_sdk.types.common import HexStr, Node
from iota_sdk.types.feature import Feature
from iota_sdk.types.node_info import ProtocolParameters
from iota_sdk.types.output import AccountOutput, BasicOutput, FoundryOutput, NftOutput, deserialize_output
from iota_sdk.types.payload import Payload
from iota_sdk.types.token_scheme import SimpleTokenScheme
from iota_sdk.types.unlock_condition import UnlockCondition


class Client(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, ClientUtils):
    """Represents an IOTA client.

    Attributes:
        handle: The handle to the inner client object.
    """

    # pylint: disable=unused-argument
    def __init__(
        self,
        primary_nodes: Optional[Union[Union[str, Node],
                                      List[Union[str, Node]]]] = None,
        nodes: Optional[Union[Union[str, Node],
                              List[Union[str, Node]]]] = None,
        protocol_parameters: Optional[ProtocolParameters] = None,
        ignore_node_health: Optional[bool] = None,
        api_timeout: Optional[timedelta] = None,
        node_sync_interval: Optional[timedelta] = None,
        quorum: Optional[bool] = None,
        min_quorum_size: Optional[int] = None,
        quorum_threshold: Optional[int] = None,
        user_agent: Optional[str] = None,
        broker_options: Optional[MqttBrokerOptions] = None,
        max_parallel_api_requests: Optional[int] = None,
        client_handle=None
    ):
        """Initialize the IOTA Client.

        **Arguments**
        primary_nodes :
            Nodes which will be tried first for all requests.
        nodes :
            A single Node URL or an array of URLs.
        ignore_node_health :
            If the node health should be ignored.
        api_timeout :
            Timeout for API requests.
        node_sync_interval :
            Interval in which nodes will be checked for their sync status and the network info gets updated.
        quorum :
            If node quorum is enabled. Will compare the responses from multiple nodes and only returns the response if 'quorum_threshold'% of the nodes return the same one.
        min_quorum_size :
            Minimum amount of nodes required for request when quorum is enabled.
        quorum_threshold :
            % of nodes that have to return the same response so it gets accepted.
        user_agent :
            The User-Agent header for requests.
        broker_options (MqttBrokerOptions):
            Options for the MQTT broker.
        max_parallel_api_requests :
            Set maximum parallel API requests.
        client_handle :
            An instance of a node client.
        """
        client_config = dict(locals())
        del client_config['self']
        # Delete client_handle, because it's not needed here and can't be
        # serialized.
        if "client_handle" in client_config:
            del client_config["client_handle"]

        client_config['primary_nodes'] = convert_nodes(primary_nodes)
        client_config['nodes'] = convert_nodes(nodes)
        if broker_options is not None:
            client_config['broker_options'] = broker_options.to_dict()
        if protocol_parameters is not None:
            client_config['protocol_parameters'] = protocol_parameters.to_dict()

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

    @_call_client_method_routine
    def _call_method(self, name, data=None):
        """Dumps json string and calls `call_client_method()`
        """
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        return message

    def get_handle(self):
        """Get the client handle.

        Returns:
            The inner client object.
        """
        return self.handle

    def build_account_output(self,
                             account_id: HexStr,
                             unlock_conditions: List[UnlockCondition],
                             amount: Optional[int] = None,
                             mana: Optional[int] = None,
                             foundry_counter: Optional[int] = None,
                             features: Optional[List[Feature]] = None,
                             immutable_features: Optional[List[Feature]] = None) -> AccountOutput:
        """Build an AccountOutput.

        Args:
            account_id: A unique ID for the new account.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            foundry_counter: A counter that denotes the number of foundries created by this account output.
            features: A list of features.
            immutable_features: A list of immutable features.

        Returns:
            The account output as dict.
        """

        return deserialize_output(self._call_method('buildAccountOutput', {
            'accountId': account_id,
            'unlockConditions': unlock_conditions,
            'amount': None if amount is None else str(amount),
            'mana': None if mana is None else str(mana),
            'foundryCounter': foundry_counter,
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_basic_output(self,
                           unlock_conditions: List[UnlockCondition],
                           amount: Optional[int] = None,
                           mana: Optional[int] = None,
                           features: Optional[List[Feature]] = None) -> BasicOutput:
        """Build a BasicOutput.

        Args:
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            features: Features that add utility to the output but do not impose unlocking conditions.

        Returns:
            The basic output as dict.
        """

        return deserialize_output(self._call_method('buildBasicOutput', {
            'unlockConditions': unlock_conditions,
            'amount': None if amount is None else str(amount),
            'mana': None if mana is None else str(mana),
            'features': features,
        }))

    def build_foundry_output(self,
                             serial_number: int,
                             token_scheme: SimpleTokenScheme,
                             unlock_conditions: List[UnlockCondition],
                             amount: Optional[int] = None,
                             features: Optional[List[Feature]] = None,
                             immutable_features: Optional[List[Feature]] = None) -> FoundryOutput:
        """Build a FoundryOutput.

        Args:
            serial_number: The serial number of the foundry with respect to the controlling account.
            token_scheme: Defines the supply control scheme of the tokens controlled by the foundry. Currently only a simple scheme is supported.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            features: Features that add utility to the output but do not impose unlocking conditions.
            immutable_features: Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.

        Returns:
            The foundry output as dict.
        """

        return deserialize_output(self._call_method('buildFoundryOutput', {
            'serialNumber': serial_number,
            'tokenScheme': token_scheme,
            'unlockConditions': unlock_conditions,
            'amount': None if amount is None else str(amount),
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def build_nft_output(self,
                         nft_id: HexStr,
                         unlock_conditions: List[UnlockCondition],
                         amount: Optional[int] = None,
                         mana: Optional[int] = None,
                         features: Optional[List[Feature]] = None,
                         immutable_features: Optional[List[Feature]] = None) -> NftOutput:
        """Build an NftOutput.

        Args:
            nft_id: A unique ID for the new NFT.
            unlock_conditions: The unlock conditions for the new output.
            amount: The amount of base coins in the new output.
            mana: Amount of stored Mana held by this output.
            features: Features that add utility to the output but do not impose unlocking conditions.
            immutable_features: Features that add utility to the output but do not impose unlocking conditions. These features need to be kept in future transitions of the UTXO state machine.

        Returns:
            The NFT output as dict.
        """

        return deserialize_output(self._call_method('buildNftOutput', {
            'nftId': nft_id,
            'unlockConditions': unlock_conditions,
            'amount': None if amount is None else str(amount),
            'mana': None if mana is None else str(mana),
            'features': features,
            'immutableFeatures': immutable_features
        }))

    def get_node(self) -> Dict[str, Any]:
        """Get a node candidate from the healthy node pool.
        """
        return self._call_method('getNode')

    def get_protocol_parameters(self) -> ProtocolParameters:
        """Gets the protocol parameters.
        """
        return ProtocolParameters.from_dict(
            self._call_method('getProtocolParameters'))

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

    def build_basic_block(
        self,
        issuer_id: HexStr,
        payload: Optional[Payload] = None,
    ) -> UnsignedBlock:
        """Build a basic block.

        Args:
            issuer_id: The identifier of the block issuer account.
            payload: The payload to submit.

        Returns:
            An unsigned block.
        """
        result = self._call_method('buildBasicBlock', {
            'issuerId': issuer_id,
            'payload': payload,
        })
        return UnsignedBlock.from_dict(result)

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


def convert_nodes(
        nodes: Optional[Union[Union[str, Node], List[Union[str, Node]]]] = None):
    """Helper function to convert provided nodes to a list for the client options.
    """
    if isinstance(nodes, list):
        return [node.to_dict() if isinstance(node, Node)
                else node for node in nodes]
    if isinstance(nodes, Node):
        return [nodes.to_dict()]
    if nodes is not None:
        return [nodes]
    return None
