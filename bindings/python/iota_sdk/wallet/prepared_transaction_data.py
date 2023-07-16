# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.transaction import Transaction


class PreparedTransactionData:
    """Helper class for offline signing.

    Attributes:
        account (account object): an account object used to continue building this transaction
        prepared_transaction_data_dto (dict of prepared data): the data of a prepared transaction object
    """

    def __init__(
        self,
        account,
        prepared_transaction_data
    ):
        """
        Args:
            account (account object): an account object used to continue building this transaction
            prepared_transaction_data (dict of prepared data): the data of a prepared transaction object
        """
        self.account = account
        self.prepared_transaction_data_dto = prepared_transaction_data

    """
    The function returns the prepared transaction data.

    :returns: The method prepared_transaction_data() is returning an object of type PreparedTransaction
    """

    def prepared_transaction_data(self):
        """Gets the prepared transaction data.

        Returns: 
            the method prepared_transaction_data() is returning an object of type PreparedTransaction
        """
        return self.prepared_transaction_data_dto

    """
    The send function returns a promise that resolves to a Transaction object after signing
    and submitting the transaction. Internally just calls sign_and_submit_transaction.

    :returns: The send() method is returning a Transaction object after it has been signed and submitted.
    """

    def send(self) -> Transaction:
        """Sends a transaction. Internally just calls `sign_and_submit_transaction`.

        Returns: 
            a Transaction object after it has been signed and submitted
        """
        return self.sign_and_submit_transaction()

    def sign(self):
        """Signs a prepared transaction essence using the account's private key and returns
        the signed transaction essence.

        Returns: 
            a SignedTransactionEssence object
        """
        return self.account.sign_transaction_essence(self.prepared_transaction_data())

    def sign_and_submit_transaction(self) -> Transaction:
        """Signs and submits a transaction using prepared transaction data.

        Returns:
            a Transaction object
        """
        return self.account.sign_and_submit_transaction(self.prepared_transaction_data())


class PreparedCreateTokenTransaction(PreparedTransactionData):
    """Represents a prepared create-native-token transaction.

    Returns: The token id of the PreparedCreateTokenTransaction.
    """

    def token_id(self):
        """Gets the native token id as a string.

        Returns:
            the corresponding native token id
        """
        return self.prepared_transaction_data_dto["tokenId"]

    def prepared_transaction_data(self):
        return self.prepared_transaction_data_dto["transaction"]
