# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import json
from iota_sdk.types.output import Output
from iota_sdk.types.transaction_id import TransactionId


class OutputId(str):
    """Represents an output ID.

    Attributes:
        output_id: The unique id of an output.
    """

    def __new__(cls, output_id: str):
        """Initialize OutputId
        """
        if len(output_id) != 78:
            raise ValueError(
                'output_id length must be 78 characters with 0x prefix')
        if not output_id.startswith('0x'):
            raise ValueError('output_id must start with 0x')
        # Validate that it has only valid hex characters
        int(output_id[2:], 16)
        instance = super().__new__(cls, output_id)
        return instance

    @classmethod
    def from_transaction_id_and_output_index(
            cls, transaction_id: TransactionId, output_index: int):
        """Creates an `OutputId` instance from its transaction id and output index.

        Args:
            transaction_id: The transaction id associated with the output.
            output_index: The index of the output within a transaction.

        Returns:
            OutputId: The unique id of an output.
        """
        if len(transaction_id) != 74:
            raise ValueError(
                'transaction_id length must be 74 characters with 0x prefix')
        if not transaction_id.startswith('0x'):
            raise ValueError('transaction_id must start with 0x')
        # Validate that it has only valid hex characters
        int(transaction_id[2:], 16)
        output_index_hex = (output_index).to_bytes(2, "little").hex()
        return OutputId(transaction_id + output_index_hex)

    def transaction_id(self) -> TransactionId:
        """Returns the TransactionId of an OutputId.
        """
        return TransactionId(self[:74])

    def output_index(self) -> int:
        """Returns the output index of an OutputId.
        """
        return int.from_bytes(
            bytes.fromhex(self[74:]), 'little')

    @classmethod
    def from_dict(cls, output_id_dict: dict):
        """Init an OutputId from a dict.
        """
        return OutputId(output_id_dict)


@json
@dataclass
class OutputWithId:
    """An Output with its ID.

    Arguments:
        output: Output,
        output_id: OutputId,
    """
    output: Output
    output_id: OutputId
