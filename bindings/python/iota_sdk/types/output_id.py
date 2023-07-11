# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr


class OutputId(dict):
    def __init__(self, transaction_id: HexStr, output_index: int):
        """Initialize OutputId
        """
        if len(transaction_id) != 66:
            raise ValueError(
                'transaction_id length must be 66 characters with 0x prefix')
        if not transaction_id.startswith('0x'):
            raise ValueError('transaction_id must start with 0x')
        # Validate that it has only valid hex characters
        int(transaction_id[2:], 16)
        if output_index not in range(0, 129):
            raise ValueError('output_index must be a value from 0 to 128')
        output_index_hex = (output_index).to_bytes(2, "little").hex()
        self.output_id = transaction_id + output_index_hex
        self.transaction_id = transaction_id
        self.output_index = output_index

    @classmethod
    def from_string(cls, output_id: HexStr):
        obj = cls.__new__(cls)
        super(OutputId, obj).__init__()
        if len(output_id) != 70:
            raise ValueError(
                'output_id length must be 70 characters with 0x prefix')
        if not output_id.startswith('0x'):
            raise ValueError('transaction_id must start with 0x')
        # Validate that it has only valid hex characters
        int(output_id[2:], 16)
        obj.output_id = output_id
        obj.transaction_id = HexStr(output_id[:66])
        obj.output_index = int.from_bytes(
            bytes.fromhex(output_id[66:]), 'little')
        return obj

    def __repr__(self):
        return self.output_id
