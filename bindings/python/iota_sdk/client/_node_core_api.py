# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional, Union
from abc import ABCMeta, abstractmethod

from iota_sdk.types.block.block import Block
from iota_sdk.types.block.metadata import BlockMetadata, BlockWithMetadata
from iota_sdk.types.committee import Committee, Validator, Validators
from iota_sdk.types.common import HexStr, EpochIndex, SlotIndex
from iota_sdk.types.issuance import Congestion, IssuanceBlockHeader
from iota_sdk.types.mana import ManaRewards
from iota_sdk.types.node_info import NodeInfo, NodeInfoWrapper, Routes
from iota_sdk.types.output import OutputWrapper
from iota_sdk.types.output_metadata import OutputWithMetadata, OutputMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.slot import SlotCommitment
from iota_sdk.types.transaction_metadata import TransactionMetadata
from iota_sdk.types.utxo_changes import UtxoChanges, UtxoChangesFull


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

    # Node routes.

    def get_health(self, url: str) -> bool:
        """Returns the health of the node.
        GET /health

        Args:
            url: The node's url.
        """
        return self._call_method('getHealth', {
            'url': url
        })

    def get_info(self) -> NodeInfoWrapper:
        """Returns general information about the node.
        GET /api/core/v3/info
        """
        return NodeInfoWrapper.from_dict(self._call_method('getInfo'))

    def get_routes(self) -> Routes:
        """Returns the available API route groups of the node.
        GET /api/routes
        """
        return Routes.from_dict(self._call_method('getRoutes'))

    # FIXME: Remove or move? (not part of official IOTA 2.0 Core API)
    def get_node_info(self, url: str, auth=None) -> NodeInfo:
        """Get node info.
        GET /api/core/v3/info endpoint

        Args:
            url: The node's url.
            auth: A JWT or username/password authentication object.

        Returns:
            The node info.
        """
        return NodeInfo.from_dict(self._call_method('getNodeInfo', {
            'url': url,
            'auth': auth
        }))

    def call_plugin_route(self, base_plugin_path: str, method: str,
                          endpoint: str, query_params: Optional[List[str]] = None, request: Optional[str] = None):
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

    # Accounts routes.

    def get_account_congestion(self, account_id: HexStr) -> Congestion:
        """Checks if the account is ready to issue a block.
        GET /api/core/v3/accounts/{bech32Address}/congestion
        """
        return self._call_method('getAccountCongestion', {
            'accountId': account_id
        })

    # Rewards routes.

    def get_output_mana_rewards(
            self, output_id: HexStr, slot_index: SlotIndex) -> ManaRewards:
        """Returns the total available Mana rewards of an account or delegation output decayed up to `epochEnd` index
        provided in the response.
        Note that rewards for an epoch only become available at the beginning of the next epoch. If the end epoch of a
        staking feature is equal or greater than the current epoch, the rewards response will not include the potential
        future rewards for those epochs. `epochStart` and `epochEnd` indicates the actual range for which reward value
        is returned and decayed for.
        GET /api/core/v3/rewards/{outputId}
        """
        return self._call_method('getOutputManaRewards', {
            'outputId': output_id,
            'slotIndex': slot_index
        })

    # Committee routes.

    def get_committee(self, epoch_index: EpochIndex) -> Committee:
        """Returns the information of committee members at the given epoch index. If epoch index is not provided, the
        current committee members are returned.
        GET /api/core/v3/committee/?epochIndex
        """
        return self._call_method('getCommittee', {
            'epochIndex': epoch_index
        })

    # Validators routes.

    def get_validators(self, page_size, cursor) -> Validators:
        """Returns information of all registered validators and if they are active.
        GET JSON to /api/core/v3/validators
        """
        return self._call_method('getValidators', {
            'pageSize': page_size,
            'cursor': cursor
        })

    def get_validator(self, account_id: HexStr) -> Validator:
        """Return information about a validator.
        GET /api/core/v3/validators/{bech32Address}
        """
        return self._call_method('getValidator', {
            'accountId': account_id
        })

    # Block routes.

    def get_issuance(self) -> IssuanceBlockHeader:
        """Returns information that is ideal for attaching a block in the network.
        GET /api/core/v3/blocks/issuance
        """
        return self._call_method('getIssuance')

    def post_block(self, block: Block) -> HexStr:
        """Returns the BlockId of the submitted block.
        POST JSON to /api/core/v3/blocks

        Args:
            block: The block to post.

        Returns:
            The block id of the posted block.
        """
        return self._call_method('postBlock', {
            'block': block
        })

    def post_block_raw(self, block: Block) -> HexStr:
        """Returns the BlockId of the submitted block.
        POST /api/core/v3/blocks

        Returns:
            The corresponding block id of the block.
        """
        return self._call_method('postBlockRaw', {
            'block': block
        })

    def get_block(self, block_id: HexStr) -> Block:
        """Finds a block by its ID and returns it as object.
        GET /api/core/v3/blocks/{blockId}

        Returns:
            The corresponding block.
        """
        return Block.from_dict(self._call_method('getBlock', {
            'blockId': block_id
        }))

    def get_block_raw(self, block_id: HexStr) -> List[int]:
        """Finds a block by its ID and returns it as raw bytes.
        GET /api/core/v3/blocks/{blockId}

        Returns:
            The corresponding raw bytes of a block.
        """
        return self._call_method('getBlockRaw', {
            'blockId': block_id
        })

    def get_block_metadata(self, block_id: HexStr) -> BlockMetadata:
        """Returns the metadata of a block.
        GET /api/core/v3/blocks/{blockId}/metadata

        Returns:
            The corresponding block metadata.
        """
        return BlockMetadata.from_dict(self._call_method('getBlockMetadata', {
            'blockId': block_id
        }))

    def get_block_with_metadata(self, block_id: HexStr) -> BlockWithMetadata:
        """Returns a block with its metadata.
        GET /api/core/v2/blocks/{blockId}/full

        Returns:
            The corresponding block with it metadata.
        """
        return BlockWithMetadata.from_dict(self._call_method('getBlockWithMetadata', {
            'blockId': block_id
        }))

    # UTXO routes.

    def get_output(
            self, output_id: Union[OutputId, HexStr]) -> OutputWrapper:
        """Finds an output by its ID and returns it as object.
        GET /api/core/v3/outputs/{outputId}

        Returns:
            The corresponding output.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return OutputWrapper.from_dict(self._call_method('getOutput', {
            'outputId': output_id_str
        }))

    def get_output_raw(
            self, output_id: Union[OutputId, HexStr]) -> List[int]:
        """Finds an output by its ID and returns it as raw bytes.
        GET /api/core/v3/outputs/{outputId}

        Returns:
            The raw bytes of the corresponding output.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return self._call_method('getOutputRaw', {
            'outputId': output_id_str
        })

    def get_output_metadata(
            self, output_id: Union[OutputId, HexStr]) -> OutputMetadata:
        """Finds output metadata by output ID.
        GET /api/core/v3/outputs/{outputId}/metadata

        Returns:
            The output metadata.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        # TODO: remove
        # return from_dict(OutputMetadata,
        # self._call_method('getOutputMetadata', {
        return OutputMetadata.from_dict(self._call_method('getOutputMetadata', {
            'outputId': output_id_str
        }))

    def get_output_with_metadata(
            self, output_id: Union[OutputId, HexStr]) -> OutputWithMetadata:
        """Finds an output with its metadata by output ID.
        GET /api/core/v3/outputs/{outputId}/full

        Returns:
            The corresponding output.
        """
        output_id_str = output_id.output_id if isinstance(
            output_id, OutputId) else output_id
        return OutputWithMetadata.from_dict(self._call_method('getOutputWithMetadata', {
            'outputId': output_id_str
        }))

    def get_included_block(self, transaction_id: HexStr) -> Block:
        """Returns the earliest confirmed block containing the transaction with the given ID.
        GET /api/core/v3/transactions/{transactionId}/included-block

        Returns:
            The included block.
        """
        return Block.from_dict(self._call_method('getIncludedBlock', {
            'transactionId': transaction_id
        }))

    def get_included_block_raw(self, transaction_id: HexStr) -> List[int]:
        """Returns the earliest confirmed block containing the transaction with the given ID, as raw bytes.
        GET /api/core/v3/transactions/{transactionId}/included-block

        Returns:
            The raw bytes of the included block.
        """
        return self._call_method('getIncludedBlockRaw', {
            'transactionId': transaction_id
        })

    def get_included_block_metadata(
            self, transaction_id: HexStr) -> BlockMetadata:
        """Returns the metadata of the earliest block containing the tx that was confirmed.
        GET /api/core/v3/transactions/{transactionId}/included-block/metadata

        Returns:
            The metadata of the included block.
        """
        return BlockMetadata.from_dict(self._call_method('getIncludedBlockMetadata', {
            'transactionId': transaction_id
        }))

    def get_transaction_metadata(
            self, transaction_id: HexStr) -> TransactionMetadata:
        """Finds the metadata of a transaction.
        GET /api/core/v3/transactions/{transactionId}/metadata

        Returns:
            The transaction metadata.
        """
        return TransactionMetadata.from_dict(self._call_method('getTransactionMetadata', {
            'transactionId': transaction_id
        }))

    # Commitments routes.

    def get_slot_commitment_by_id(
            self, slot_commitment_id: HexStr) -> SlotCommitment:
        """Finds a slot commitment by its ID and returns it as object.
        GET /api/core/v3/commitments/{commitmentId}

        Returns:
            The corresponding slot commitment.
        """
        return SlotCommitment.from_dict(self._call_method('getSlotCommitmentById', {
            'slotCommitmentId': slot_commitment_id
        }))

    def get_slot_commitment_by_id_raw(
            self, slot_commitment_id: HexStr) -> List[int]:
        """Finds a slot commitment by its ID and returns it as raw bytes.
        GET /api/core/v3/commitments/{commitmentId}

        Returns:
            The raw bytes of the corresponding slot commitment.
        """
        return self._call_method('getSlotCommitmentByIdRaw', {
            'slotCommitmentId': slot_commitment_id
        })

    def get_utxo_changes_by_slot_commitment_id(
            self, slot_commitment_id: HexStr) -> UtxoChanges:
        """Get all UTXO changes of a given slot by slot commitment ID.
        GET /api/core/v3/commitments/{commitmentId}/utxo-changes

        Returns:
            The corresponding UTXO changes.
        """
        return UtxoChanges.from_dict(self._call_method('getUtxoChangesBySlotCommitmentId', {
            'slotCommitmentId': slot_commitment_id
        }))

    def get_utxo_changes_full_by_slot_commitment_id(
            self, slot_commitment_id: HexStr) -> UtxoChangesFull:
        """Get all full UTXO changes of a given slot by slot commitment ID.
        GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full

        Returns:
            The full UTXO changes.
        """
        return UtxoChangesFull.from_dict(self._call_method('getUtxoChangesFullBySlotCommitmentId', {
            'slotCommitmentId': slot_commitment_id
        }))

    def get_slot_commitment_by_slot(
            self, slot_index: SlotIndex) -> SlotCommitment:
        """Finds a slot commitment by slot index and returns it as object.
        GET /api/core/v3/commitments/by-slot/{slot}

        Returns:
            The corresponding slot commitment.
        """
        return SlotCommitment.from_dict(self._call_method('getSlotCommitmentBySlot', {
            'slotIndex': slot_index
        }))

    def get_slot_commitment_by_slot_raw(
            self, slot_index: SlotIndex) -> List[int]:
        """Finds a slot commitment by slot index and returns it as raw bytes.
        GET /api/core/v3/commitments/by-slot/{slot}

        Returns:
            The raw bytes of the corresponding slot commitment.
        """
        return self._call_method('getSlotCommitmentBySlotRaw', {
            'slotIndex': slot_index
        })

    def get_utxo_changes_by_slot(self, slot_index: SlotIndex) -> UtxoChanges:
        """Get all UTXO changes of a given slot by its index.
        GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes

        Returns:
            The corresponding UTXO changes.
        """
        return UtxoChanges.from_dict(self._call_method('getUtxoChangesBySlot', {
            'slotIndex': slot_index
        }))

    def get_utxo_changes_full_by_slot(
            self, slot_index: SlotIndex) -> UtxoChangesFull:
        """Get all full UTXO changes of a given slot by its index.
        GET /api/core/v3/commitments/by-slot/{slot}/utxo-changes/full

        Returns:
            The full UTXO changes.
        """
        return UtxoChangesFull.from_dict(self._call_method('getUtxoChangesFullBySlot', {
            'slotIndex': slot_index
        }))
