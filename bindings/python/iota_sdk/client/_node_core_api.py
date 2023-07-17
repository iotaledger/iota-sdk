# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.block import Block, BlockMetadata
from iota_sdk.types.common import HexStr
from iota_sdk.types.node_info import NodeInfo, NodeInfoWrapper
from iota_sdk.types.output import OutputWithMetadata, OutputMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.payload import MilestonePayload
from typing import List
from dacite import from_dict

class NodeCoreAPI():

    def get_health(self, url: str):
        """ Get node health.
        """
        return self._call_method('getHealth', {
            'url': url
        })

    def get_node_info(self, url: str, auth=None) -> NodeInfo:
        """Get node info.
        """
        return from_dict(NodeInfo, self._call_method('getNodeInfo', {
            'url': url,
            'auth': auth
        }))

    def get_info(self) -> NodeInfoWrapper:
        """Returns the node information together with the url of the used node.
        """
        return from_dict(NodeInfoWrapper, self._call_method('getInfo'))

    def get_peers(self):
        """Get peers.
        """
        return self._call_method('getPeers')

    def get_tips(self) -> List[HexStr]:
        """Get tips.
        """
        return self._call_method('getTips')

    def post_block(self, block: Block) -> HexStr:
        """Post block.
        """
        return self._call_method('postBlock', {
            'block': block.__dict__
        })

    def get_block_data(self, block_id: HexStr) -> Block:
        """Get a block.
        """
        return Block.from_dict(self._call_method('getBlock', {
            'blockId': block_id
        }))

    def get_block_metadata(self, block_id: HexStr) -> BlockMetadata:
        """Get block metadata with block_id.
        """
        return BlockMetadata.from_dict(self._call_method('getBlockMetadata', {
            'blockId': block_id
        }))

    def get_block_raw(self, block_id: HexStr) -> List[int]:
        """Get block raw.
        """
        return self._call_method('getBlockRaw', {
            'blockId': block_id
        })

    def post_block_raw(self, block_bytes: List[int]) -> HexStr:
        """Post block raw.
        """
        return self._call_method('postBlockRaw', {
            'blockBytes': block_bytes
        })

    def get_output(self, output_id: OutputId) -> OutputWithMetadata:
        """Get output.
        """
        return from_dict(OutputWithMetadata, self._call_method('getOutput', {
            'outputId': output_id
        }))

    def get_output_metadata(self, output_id: OutputId) -> OutputMetadata:
        """Get output metadata.
        """
        return from_dict(OutputMetadata, self._call_method('getOutputMetadata', {
            'outputId': output_id
        }))

    def get_milestone_by_id(self, milestone_id: HexStr) -> MilestonePayload:
        """Get the milestone by the given milestone id.
        """
        result = self._call_method('getMilestoneById', {
            'milestoneId': milestone_id
        })
        return MilestonePayload.from_dict(result)

    def get_milestone_by_id_raw(self, milestone_id: HexStr) -> List[int]:
        """Get the raw milestone by the given milestone id.
        """
        return self._call_method('getMilestoneByIdRaw', {
            'milestoneId': milestone_id
        })

    def get_milestone_by_index(self, index: int) -> MilestonePayload:
        """Get the milestone by the given index.
        """
        result = self._call_method('getMilestoneByIndex', {
            'index': index
        })
        return MilestonePayload.from_dict(result)

    def get_milestone_by_index_raw(self, index: int) -> List[int]:
        """Get the milestone by the given index.
        """
        return self._call_method('getMilestoneByIndexRaw', {
            'index': index
        })

    def get_utxo_changes_by_id(self, milestone_id: HexStr):
        """Get the UTXO changes by the given milestone id.
        """
        return self._call_method('getUtxoChangesById', {
            'milestoneId': milestone_id
        })

    def get_utxo_changes_by_index(self, index: int):
        """Get the UTXO changes by the given milestone index.
        """
        return self._call_method('getUtxoChangesByIndex', {
            'index': index
        })

    def get_receipts(self):
        """Get all receipts.
        """
        return self._call_method('getReceipts')

    def get_receipts_migrated_at(self, milestone_index: int):
        """Get the receipts by the given milestone index.
        """
        return self._call_method('getReceiptsMigratedAt', {
            'milestoneIndex': milestone_index
        })

    def get_treasury(self):
        """Get the treasury output.
        """
        return self._call_method('getTreasury')

    def get_included_block(self, transaction_id: HexStr) -> Block:
        """Returns the included block of the transaction.
        """
        return Block.from_dict(self._call_method('getIncludedBlock', {
            'transactionId': transaction_id
        }))

    def get_included_block_metadata(self, transaction_id: HexStr) -> BlockMetadata:
        """Returns the metadata of the included block of the transaction.
        """
        return BlockMetadata.from_dict(self._call_method('getIncludedBlockMetadata', {
            'transactionId': transaction_id
        }))

    def call_plugin_route(self, base_plugin_path: str, method: str, endpoint: str, query_params: [str] = None, request: str = None):
        """Extension method which provides request methods for plugins.
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
