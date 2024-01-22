# from typing import get_args
import pytest
from iota_sdk import IssuanceBlockHeader


def test_issuance_block_header():
    # regular serialization (omitted optional fields)
    issuance_dict1 = {
        "strongParents": [
            "0x17c297a273facf4047e244a65eb34ee33b1f1698e1fff28679466fa2ad81c0e8",
            "0x9858e80fa0b37b6d9397e23d1f58ce53955a9be1aa8020c0d0e11672996c6db9"],
        "latestParentBlockIssuingTime": "180500",
        "latestFinalizedSlot": 10,
        "latestCommitment": {
            "protocolVersion": 3,
            "slot": 986,
            "previousCommitmentId": "0xd91549dbf441da4c7e50063b58c05d5b2bfb1d33b346a2f9a18220ce6207f7a289010000",
            "rootsId": "0x6a8f424929e1f08d87a204efc8a60499b789465aaa28178b77debe75cc2915d3",
            "cumulativeWeight": "78901",
            "referenceManaCost": "600"
        }
    }
    issuance1 = IssuanceBlockHeader.from_dict(issuance_dict1)
    assert "0x17c297a273facf4047e244a65eb34ee33b1f1698e1fff28679466fa2ad81c0e8" in issuance1.strong_parents
    assert issuance1.latest_parent_block_issuing_time == 180500
    assert issuance1.latest_finalized_slot == 10
    assert issuance1.latest_commitment.protocol_version == 3
    assert issuance1.latest_commitment.slot == 986
    assert issuance1.latest_commitment.previous_commitment_id == "0xd91549dbf441da4c7e50063b58c05d5b2bfb1d33b346a2f9a18220ce6207f7a289010000"
    assert issuance1.latest_commitment.roots_id == "0x6a8f424929e1f08d87a204efc8a60499b789465aaa28178b77debe75cc2915d3"
    assert issuance1.latest_commitment.cumulative_weight == 78901
    assert issuance1.latest_commitment.reference_mana_cost == 600
    assert issuance1.to_dict() == issuance_dict1

    # a serialization the implementation has to be able to deal with (present optional fields with empty lists)
    issuance_dict2 = {
        "strongParents": [
            "0x17c297a273facf4047e244a65eb34ee33b1f1698e1fff28679466fa2ad81c0e8",
            "0x9858e80fa0b37b6d9397e23d1f58ce53955a9be1aa8020c0d0e11672996c6db9"],
        "weakParents": [],
        "shallowLikeParents": [],
        "latestParentBlockIssuingTime": "180500",
        "latestFinalizedSlot": 10,
        "latestCommitment": {
            "protocolVersion": 3,
            "slot": 986,
            "previousCommitmentId": "0xd91549dbf441da4c7e50063b58c05d5b2bfb1d33b346a2f9a18220ce6207f7a289010000",
            "rootsId": "0x6a8f424929e1f08d87a204efc8a60499b789465aaa28178b77debe75cc2915d3",
            "cumulativeWeight": "78901",
            "referenceManaCost": "600"
        }
    }
    assert IssuanceBlockHeader.from_dict(issuance_dict2).to_dict() == issuance_dict2
