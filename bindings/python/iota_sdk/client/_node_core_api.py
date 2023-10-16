# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Union
from abc import ABCMeta, abstractmethod
from dacite import from_dict

from iota_sdk.types.block import Block, BlockMetadata
from iota_sdk.types.common import HexStr
from iota_sdk.types.node_info import NodeInfo, NodeInfoWrapper
from iota_sdk.types.output import OutputWithMetadata, OutputMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import MilestonePayload
from iota_sdk.types.utxo_changes import UtxoChanges


class NodeCoreAPI(metaclass=ABCMeta):
    """Node core API.
    """

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

    def get_health(self, url: str):
        """ Get node health.

        Args:
            url: The node's url.
        """
        return self._call_method('getHealth', {
            'url': url
        })

    def get_node_info(self, url: str, auth=None) -> NodeInfo:
        """Get node info.

        Args:
            url: The node's url.
            auth: A JWT or username/password authentication object.
        """
        return from_dict(NodeInfo, self._call_method('getNodeInfo', {
            'url': url,
            'auth': auth
        }))

    def get_info(self) -> NodeInfoWrapper:
        """Return node information together with the url of the used node.
        """
        return from_dict(NodeInfoWrapper, self._call_method('getInfo'))

    def get_peers(self):
        """Get the peers of the node.
        """
        return self._call_method('getPeers')

    def get_tips(self) -> List[HexStr]:
        """Request tips from the node.
        """
        return self._call_method('getTips')

    def post_block(self, block: Block) -> HexStr:
        """Post a block.

        Args:
            block: The block to post.

        Returns:
            The block id of the posted block.
        """
        return self._call_method('postBlock', {
            'block': block.__dict__
        })

    def get_block_data(self, block_id: HexStr) -> Block:
        """Get the block corresponding to the given block id.
        """
        return Block.from_dict(self._call_method('getBlock', {
            'blockId': block_id
        }))

    def get_block_metadata(self, block_id: HexStr) -> BlockMetadata:
        """Get the block metadata corresponding to the given block id.
        """
        return BlockMetadata.from_dict(self._call_method('getBlockMetadata', {
            'blockId': block_id
        }))

    def get_block_raw(self, block_id: HexStr) -> List[int]:
        """Get the raw bytes of the block corresponding to the given block id.
        """
        return self._call_method('getBlockRaw', {
            'blockId': block_id
        })

    def post_block_raw(self, block_bytes: List[int]) -> HexStr:
        """Post a block as raw bytes.

        Returns:
            The corresponding block id of the block.
        """
        return self._call_method('postBlockRaw', {
            'blockBytes': block_bytes
        })

    def get_output(
            self, output_id: Union[OutputId, HexStr]) -> OutputWithMetadata:
        """Get the output corresponding to the given output id.

        Returns:
            The output itself with its metadata.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return from_dict(OutputWithMetadata, self._call_method('getOutput', {
            'outputId': output_id_str
        }))

    def get_output_metadata(
            self, output_id: Union[OutputId, HexStr]) -> OutputMetadata:
        """Get the output metadata corresponding to the given output id.

        Returns:
            The output metadata.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return from_dict(OutputMetadata, self._call_method('getOutputMetadata', {
            'outputId': output_id_str
        }))

    def get_milestone_by_id(self, milestone_id: HexStr) -> MilestonePayload:
        """Get the milestone corresponding to the given milestone id.

        Returns:
            The milestone payload.
        """
        result = self._call_method('getMilestoneById', {
            'milestoneId': milestone_id
        })
        return MilestonePayload.from_dict(result)

    def get_milestone_by_id_raw(self, milestone_id: HexStr) -> List[int]:
        """Get the raw bytes of the milestone corresponding to the given milestone id.

        Returns:
            The raw bytes of the milestone.
        """
        return self._call_method('getMilestoneByIdRaw', {
            'milestoneId': milestone_id
        })

    def get_milestone_by_index(self, index: int) -> MilestonePayload:
        """Get the milestone by the given milestone index.

        Returns:
            The milestone payload.
        """
        result = self._call_method('getMilestoneByIndex', {
            'index': index
        })
        return MilestonePayload.from_dict(result)

    def get_milestone_by_index_raw(self, index: int) -> List[int]:
        """Get the raw bytes of the milestone corresponding to the given milestone index.

        Returns:
            The raw bytes of the milestone.
        """
        return self._call_method('getMilestoneByIndexRaw', {
            'index': index
        })

    def get_utxo_changes_by_id(self, milestone_id: HexStr) -> UtxoChanges:
        """Get the UTXO changes applied in the given milestone.
        """
        return from_dict(UtxoChanges, self._call_method('getUtxoChangesById', {
            'milestoneId': milestone_id
        }))

    def get_utxo_changes_by_index(self, index: int) -> UtxoChanges:
        """Get the UTXO changes applied at the given milestone index.
        """
        return from_dict(UtxoChanges, self._call_method('getUtxoChangesByIndex', {
            'index': index
        }))

    def get_receipts(self):
        """Get all receipts.
        """
        return self._call_method('getReceipts')

    def get_receipts_migrated_at(self, milestone_index: int):
        """Get the receipts that were migrated at the given milestone index.
        """
        return self._call_method('getReceiptsMigratedAt', {
            'milestoneIndex': milestone_index
        })

    def get_treasury(self):
        """Get the treasury output.
        """
        return self._call_method('getTreasury')

    def get_included_block(self, transaction_id: HexStr) -> Block:
        """Returns the included block of the given transaction.

        Returns:
            The included block.
        """
        return Block.from_dict(self._call_method('getIncludedBlock', {
            'transactionId': transaction_id
        }))

    def get_included_block_metadata(
            self, transaction_id: HexStr) -> BlockMetadata:
        """Returns the metadata of the included block of the given transaction.

        Returns:
            The metadata of the included block.
        """
        return BlockMetadata.from_dict(self._call_method('getIncludedBlockMetadata', {
            'transactionId': transaction_id
        }))

    def call_plugin_route(self, base_plugin_path: str, method: str,
                          endpoint: str, query_params: [str] = None, request: str = None):
        """Extension method which provides request methods for plugins.

        Args:
            base_plugin_path: The base path of the routes provided by the plugin.
            method: The HTTP method.
            endpoint: The endpoint to query provided by the plugin.
            query_params: The parameters of the query.
            request: The request object sent to the endpoint of the plugin.
        """
        if query_params is None:
            query_params = []
        return self._call_method('callPluginRoute', {
            'basePluginPath': base_plugin_path,
            'method': method,
            'endpoint': endpoint,
            'queryParams': query_params,
            'request': request,
        })
