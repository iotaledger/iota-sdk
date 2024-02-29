# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import TYPE_CHECKING
from dataclasses import dataclass
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.transaction_with_metadata import CreateDelegationTransaction, CreateNativeTokenTransaction, TransactionWithMetadata
from iota_sdk.types.transaction_data import PreparedTransactionData

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.wallet.wallet import Wallet


@dataclass
class PreparedTransaction:
    """A helper class for offline signing.

    Attributes:
        wallet: A wallet object used to continue building this transaction.
        prepared_transaction_data: A prepared transaction data object.
    """
    wallet: Wallet
    prepared_transaction_data: PreparedTransactionData

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
            self.prepared_transaction_data)

    def sign_and_submit_transaction(self) -> TransactionWithMetadata:
        """Sign and submit a transaction using prepared transaction data.

        Returns:
            The transaction after it has been signed and submitted.
        """
        return self.wallet.sign_and_submit_transaction(
            self.prepared_transaction_data)


@dataclass
class PreparedCreateTokenTransaction:
    """A helper class for offline signing a create native token transaction.

    Attributes:
        wallet: A wallet object used to continue building this transaction.
        prepared_transaction_data: A prepared transaction data object.
    """
    wallet: Wallet
    prepared_transaction_data: PreparedCreateTokenTransactionData

    def send(self) -> CreateNativeTokenTransaction:
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
            self.prepared_transaction_data.transaction)

    def sign_and_submit_transaction(self) -> CreateNativeTokenTransaction:
        """Sign and submit a transaction using prepared transaction data.

        Returns:
            The transaction after it has been signed and submitted.
        """
        tx = self.wallet.sign_and_submit_transaction(
            self.prepared_transaction_data.transaction)
        CreateNativeTokenTransaction(
            self.prepared_transaction_data.token_id, tx)


@json
@dataclass
class PreparedCreateTokenTransactionData:
    """Prepared transaction data for creating a native token.

    Attributes:
        token_id: The token id.
        transaction: The transaction that will create the delegation.
    """
    token_id: HexStr
    transaction: PreparedTransactionData


@dataclass
class PreparedCreateDelegationTransaction:
    """A helper class for offline signing to create a delegation transaction.

    Attributes:
        wallet: A wallet object used to continue building this transaction.
        prepared_transaction_data: A prepared transaction data object.
    """
    wallet: Wallet
    prepared_transaction_data: PreparedCreateDelegationTransactionData

    def send(self) -> CreateDelegationTransaction:
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
            self.prepared_transaction_data.transaction)

    def sign_and_submit_transaction(self) -> CreateDelegationTransaction:
        """Sign and submit a transaction using prepared transaction data.

        Returns:
            The transaction after it has been signed and submitted.
        """
        tx = self.wallet.sign_and_submit_transaction(
            self.prepared_transaction_data.transaction)
        CreateDelegationTransaction(
            self.prepared_transaction_data.delegation_id, tx)


@json
@dataclass
class PreparedCreateDelegationTransactionData:
    """Prepared transaction data for creating a delegation.

    Attributes:
        delegation_id: The id of the delegation that will be created.
        transaction: The transaction that will create the delegation.
    """
    delegation_id: HexStr
    transaction: PreparedTransactionData
