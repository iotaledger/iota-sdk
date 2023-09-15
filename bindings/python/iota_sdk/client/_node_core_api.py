# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List
from dacite import from_dict

from iota_sdk.types.block import Block, BlockMetadata
from iota_sdk.types.common import HexStr
from iota_sdk.types.node_info import NodeInfo, NodeInfoWrapper
from iota_sdk.types.output import OutputWithMetadata, OutputMetadata
from iota_sdk.types.output_id import OutputId


class NodeCoreAPI():
    """Node core API.
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
        return NodeInfo.from_dict(self._call_method('getNodeInfo', {
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

    def get_output(self, output_id: OutputId | HexStr) -> OutputWithMetadata:
        """Get the output corresponding to the given output id.

        Returns:
            The output itself with its metadata.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return OutputWithMetadata.from_dict(self._call_method('getOutput', {
            'outputId': output_id_str
        }))

    def get_output_metadata(self, output_id: OutputId |
                            HexStr) -> OutputMetadata:
        """Get the output metadata corresponding to the given output id.

        Returns:
            The output metadata.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return from_dict(OutputMetadata, self._call_method('getOutputMetadata', {
            'outputId': output_id_str
        }))

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
