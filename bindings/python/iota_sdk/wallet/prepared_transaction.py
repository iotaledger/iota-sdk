# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations

from typing import TYPE_CHECKING, Dict, Union
from dacite import from_dict

from iota_sdk.types.transaction_with_metadata import TransactionWithMetadata
from iota_sdk.types.transaction_data import PreparedTransactionData

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.wallet.wallet import Wallet


class PreparedTransaction:
    """A helper class for offline signing.

    Attributes:
        wallet: An wallet object used to continue building this transaction.
        prepared_transaction_data_dto: A prepared transaction data object.
    """

    def __init__(
        self,
        wallet: Wallet,
        prepared_transaction_data: Union[PreparedTransactionData, Dict]
    ):
        """Initialize `Self`.
        """
        self.wallet = wallet
        self.prepared_transaction_data_dto = prepared_transaction_data

    def prepared_transaction_data(self) -> PreparedTransactionData:
        """The function returns the prepared transaction data.

        Returns:
            The method prepared_transaction_data() is returning an object of type PreparedTransaction
        """
        return self.prepared_transaction_data_dto if isinstance(
            self.prepared_transaction_data_dto, PreparedTransactionData) else from_dict(
            PreparedTransactionData, self.prepared_transaction_data_dto)

    def send(self) -> TransactionWithMetadata:
        """Send a transaction. Internally just calls `sign_and_submit_transaction`.

        Returns:
            The transaction after it has been signed and submitted.
        """
        return self.sign_and_submit_transaction()

    def sign(self):
        """Sign a prepared transaction using the wallet's private key and returns
        the signed transaction.
        """
        return self.wallet.sign_transaction(
            self.prepared_transaction_data())

    def sign_and_submit_transaction(self) -> TransactionWithMetadata:
        """Sign and submit a transaction using prepared transaction data.

        Returns:
            The transaction after it has been signed and submitted.
        """
        return self.wallet.sign_and_submit_transaction(
            self.prepared_transaction_data())


class PreparedCreateTokenTransaction(PreparedTransaction):

    """A prepared transaction for creating a native token.

    Returns: The token id of the PreparedCreateTokenTransaction.
    """

    def token_id(self):
        """Get the native token id as a string.
        """
        return self.prepared_transaction_data_dto["tokenId"]

    def prepared_transaction_data(self):
        """Returns the prepared transaction data.
        """
        return from_dict(PreparedTransactionData,
                         self.prepared_transaction_data_dto["transaction"])
