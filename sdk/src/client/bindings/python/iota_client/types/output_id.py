class OutputId():
    def __init__(self, transaction_id, output_index):
        """Initialise OutputId

        Parameters
        ----------
        transaction_id : string
        output_index : int
        """
        output_index_hex = (output_index).to_bytes(2, "little").hex()
        self.output_id = transaction_id + output_index_hex
        self.transaction_id = transaction_id
        self.output_index = output_index

    def __repr__(self):
        return self.output_id
