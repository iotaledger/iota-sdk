class OutputId():
    def __init__(self, transaction_id, output_index):
        """Initialise OutputId

        Parameters
        ----------
        transaction_id : string
        output_index : int
        """
        if len(transaction_id) != 66:
            raise ValueError(
                'transaction_id length must be 66 characters with 0x prefix')
        # Validate that it has only valid hex characters
        int(transaction_id[2:], 16)
        if output_index not in range(0, 129):
            raise ValueError('output_index must be a value from 0 to 128')
        output_index_hex = (output_index).to_bytes(2, "little").hex()
        self.output_id = transaction_id + output_index_hex
        self.transaction_id = transaction_id
        self.output_index = output_index

    @classmethod
    def from_string(cls, output_id):
        obj = cls.__new__(cls)
        super(OutputId, obj).__init__()
        if len(output_id) != 70:
            raise ValueError(
                'output_id length must be 70 characters with 0x prefix')
        # Validate that it has only valid hex characters
        int(output_id[2:], 16)
        obj.output_id = output_id
        obj.transaction_id = output_id[:66]
        obj.output_index = int.from_bytes(
            bytes.fromhex(output_id[66:]), 'little')
        return obj

    def __repr__(self):
        return self.output_id
