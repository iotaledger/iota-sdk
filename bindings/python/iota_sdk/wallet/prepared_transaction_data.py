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
        account : string or array of string
            A single Node URL or an array of URLs.
        preparedTransactionDataDto : string or array of string
            A single Node URL or an array of URLs.
        """
        self.account = account
        self.prepared_transaction_data = prepared_transaction_data

    def finish(self):
        return self.sign_and_submit_transaction()

    def sign(self):
        return self.account.sign_transaction_essence(self.prepared_transaction_data)

    def sign_and_submit_transaction(self):
        print(f'{self.prepared_transaction_data}')
        return self.account.sign_and_submit_transaction(self.prepared_transaction_data)

    def submit_and_store_transaction(self):
        return self.account.submit_and_store_transaction(self.prepared_transaction_data)