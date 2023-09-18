# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from iota_sdk.types.transaction import Transaction
from iota_sdk.types.transaction_data import PreparedTransactionData
from dacite import from_dict
from typing import TYPE_CHECKING, Dict, Union
# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.wallet.wallet import Account


class PreparedTransaction:
    """A helper class for offline signing.

    Attributes:
        account: An account object used to continue building this transaction.
        prepared_transaction_data_dto: A prepared transaction data object.
    """

    def __init__(
        self,
        account: Account,
        prepared_transaction_data: Union[PreparedTransactionData, Dict]
    ):
        """Initalize `Self`.
        """
        self.account = account
        self.prepared_transaction_data_dto = prepared_transaction_data

    """
    The function returns the prepared transaction data.

    :returns: The method prepared_transaction_data() is returning an object of type PreparedTransaction
    """

    def prepared_transaction_data(self) -> PreparedTransactionData:
        """Get the prepared transaction data.
        """
        return self.prepared_transaction_data_dto if isinstance(
            self.prepared_transaction_data_dto, PreparedTransactionData) else from_dict(PreparedTransactionData, self.prepared_transaction_data_dto)

    """
    The send function returns a promise that resolves to a Transaction object after signing
    and submitting the transaction. Internally just calls sign_and_submit_transaction.

    :returns: The send() method is returning a Transaction object after it has been signed and submitted.
    """

    def send(self) -> Transaction:
        """Send a transaction. Internally just calls `sign_and_submit_transaction`.

        Returns:
            The transaction after it has been signed and submitted.
        """
        return self.sign_and_submit_transaction()

    def sign(self):
        """Sign a prepared transaction essence using the account's private key and returns
        the signed transaction essence.
        """
        return self.account.sign_transaction_essence(
            self.prepared_transaction_data())

    def sign_and_submit_transaction(self) -> Transaction:
        """Sign and submit a transaction using prepared transaction data.

        Returns:
            The transaction after it has been signed and submitted.
        """
        return self.account.sign_and_submit_transaction(
            self.prepared_transaction_data())


class PreparedCreateTokenTransaction(PreparedTransaction):

    """A prepared transaction for creating a native token.
    The function returns the token_id as a string.

    Returns: The token id of the PreparedCreateTokenTransaction.
    """

    def token_id(self):
        """Get the native token id as a string.
        """
        return self.prepared_transaction_data_dto["tokenId"]

    def prepared_transaction_data(self):
        """The function returns the prepared transaction data.
        """
        return from_dict(PreparedTransactionData,
                         self.prepared_transaction_data_dto["transaction"])
