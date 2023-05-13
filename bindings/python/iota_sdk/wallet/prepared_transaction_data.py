# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

class PreparedTransactionData:
    def __init__(
        self,
        account,
        prepared_transaction_data
    ):
        """Initialize the IOTA Client.

        Parameters
        ----------
        account : account object
            An account object used to continue building this transaction.
        prepared_transaction_data : dict of prepared data
            The data of a prepared transaction object
        """
        self.account = account
        self.prepared_transaction_data_dto = prepared_transaction_data

    def prepared_transaction_data(self):
        return self.prepared_transaction_data_dto

    def finish(self):
        return self.sign_and_submit_transaction()

    def sign(self):
        return self.account.sign_transaction_essence(self.prepared_transaction_data())

    def sign_and_submit_transaction(self):
        return self.account.sign_and_submit_transaction(self.prepared_transaction_data())

class PreparedMintTokenTransaction(PreparedTransactionData):

    def token_id(self):
        return self.prepared_transaction_data_dto["tokenId"]

    def prepared_transaction_data(self):
        return self.prepared_transaction_data_dto["transaction"]
