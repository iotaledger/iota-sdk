# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from iota_sdk.types.output_id import OutputId
from typing import List


class NodeCoreAPI():

    def get_health(self, url: str):
        """ Get node health.
        """
        return self._call_method('getHealth', {
            'url': url
        })

    def get_node_info(self, url: str, auth=None):
        """Get node info.
        """
        return self._call_method('getNodeInfo', {
            'url': url,
            'auth': auth
        })

    def get_info(self):
        """Returns the node information together with the url of the used node.
        """
        return self._call_method('getInfo')

    def get_peers(self):
        """Get peers.
        """
        return self._call_method('getPeers')

    def get_tips(self) -> List[HexStr]:
        """Get tips.
        """
        return self._call_method('getTips')

    def post_block(self, block):
        """Post block.
        """
        return self._call_method('postBlockJson', {
            'block': block
        })

    def get_block_data(self, block_id: HexStr):
        """Post block.
        """
        return self._call_method('getBlock', {
            'blockId': block_id
        })

    def get_block_metadata(self, block_id: HexStr):
        """Get block metadata with block_id.
        """
        return self._call_method('getBlockMetadata', {
            'blockId': block_id
        })

    def get_block_raw(self, block_id: HexStr):
        """Get block raw.
        """
        return self._call_method('getBlockRaw', {
            'blockId': block_id
        })

    def post_block_raw(self, block_bytes):
        """Post block raw.
        """
        return self._call_method('postBlockRaw', {
            'blockBytes': block_bytes
        })

    def get_output(self, output_id: OutputId):
        """Get output.
        """
        return self._call_method('getOutput', {
            'outputId': output_id
        })

    def get_output_metadata(self, output_id: OutputId):
        """Get output metadata.
        """
        return self._call_method('getOutputMetadata', {
            'outputId': output_id
        })

    def get_included_block(self, transaction_id):
        """Returns the included block of the transaction.
        """
        return self._call_method('getIncludedBlock', {
            'transactionId': transaction_id
        })

    def get_included_block_metadata(self, transaction_id: HexStr):
        """Returns the metadata of the included block of the transaction.
        """
        return self._call_method('getIncludedBlockMetadata', {
            'transactionId': transaction_id
        })
