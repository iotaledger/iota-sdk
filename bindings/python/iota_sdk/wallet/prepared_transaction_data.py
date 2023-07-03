# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.transaction import Transaction

class PreparedTransactionData:
    def __init__(
        self,
        account,
        prepared_transaction_data
    ):
        """Helper struct for offline signing

        Parameters
        ----------
        account : account object
            An account object used to continue building this transaction.
        prepared_transaction_data : dict of prepared data
            The data of a prepared transaction object
        """
        self.account = account
        self.prepared_transaction_data_dto = prepared_transaction_data

    
    """
    The function returns the prepared transaction data.

    :returns: The method prepared_transaction_data() is returning an object of type PreparedTransaction
    """
    def prepared_transaction_data(self):
        return self.prepared_transaction_data_dto


    """
    The send function returns a promise that resolves to a Transaction object after signing
    and submitting the transaction. Internally just calls sign_and_submit_transaction.

    :returns: The send() method is returning a Transaction object after it has been signed and submitted.
    """
    def send(self) -> Transaction:
        return self.sign_and_submit_transaction()


    """
    This function signs a prepared transaction essence using the account's private key and returns
    the signed transaction essence.

    :returns: A SignedTransactionEssence object.
    """
    def sign(self):
        return self.account.sign_transaction_essence(self.prepared_transaction_data())

    
    """
    This function signs and submits a transaction using prepared transaction data.

    Returns:

    :returns: A Transaction object.
    """
    def sign_and_submit_transaction(self) -> Transaction:
        return self.account.sign_and_submit_transaction(self.prepared_transaction_data())

class PreparedCreateTokenTransaction(PreparedTransactionData):

    """
    The function returns the token_id as a string.

    :returns: The token id of the PreparedCreateTokenTransaction.
    """
    def token_id(self):
        return self.prepared_transaction_data_dto["tokenId"]

    def prepared_transaction_data(self):
        return self.prepared_transaction_data_dto["transaction"]
