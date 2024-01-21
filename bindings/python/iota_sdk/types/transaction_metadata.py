# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
#from dataclasses import dataclass, field
# from dataclasses_json import config
from iota_sdk.types.block.metadata import TransactionState, TransactionFailureReason
from iota_sdk.types.common import HexStr, json

@json
@dataclass
class TransactionMetadata:
    """Response of a GET transaction metadata REST API call.

    Attributes:
        transaction_id: TODO
        transaction_state: TODO
        transaction_failure_reason: TODO
    """
    transaction_id: HexStr
    transaction_state: TransactionState
    # transaction_state: TransactionState = field(metadata=config(
    #     encoder=TODO
    # ))
    transaction_failure_reason: TransactionFailureReason
    # transaction_failure_reason: TransactionFailureReason = field(metadata=config(
    #     encoder=TODO
    # ))
