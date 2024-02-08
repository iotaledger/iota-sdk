# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from iota_sdk.types.transaction_id import TransactionId


class OutputId(dict):
    """Represents an output ID.

    Attributes:
        output_id: The unique id of an output.
        transaction_id: The transaction id associated with the output.
        output_index: The index of the output within a transaction.

    """

    def __init__(self, transaction_id: TransactionId, output_index: int):
        """Initialize OutputId
        """
        if len(transaction_id) != 74:
            raise ValueError(
                'transaction_id length must be 74 characters with 0x prefix')
        if not transaction_id.startswith('0x'):
            raise ValueError('transaction_id must start with 0x')
        # Validate that it has only valid hex characters
        int(transaction_id[2:], 16)
        output_index_hex = (output_index).to_bytes(2, "little").hex()
        self.output_id = transaction_id + output_index_hex
        self.transaction_id = transaction_id
        self.output_index = output_index

    @classmethod
    def from_string(cls, output_id: HexStr):
        """Creates an `OutputId` instance from a `HexStr`.

        Args:
            output_id: The unique id of an output as a hex string.

        Returns:
            OutputId: The unique id of an output.
        """
        obj = cls.__new__(cls)
        super(OutputId, obj).__init__()
        if len(output_id) != 78:
            raise ValueError(
                'output_id length must be 78 characters with 0x prefix')
        if not output_id.startswith('0x'):
            raise ValueError('transaction_id must start with 0x')
        # Validate that it has only valid hex characters
        int(output_id[2:], 16)
        obj.output_id = output_id
        obj.transaction_id = TransactionId(output_id[:74])
        obj.output_index = int.from_bytes(
            bytes.fromhex(output_id[74:]), 'little')
        return obj

    def __repr__(self):
        return self.output_id
