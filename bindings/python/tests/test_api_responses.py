# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import Generic, TypeVar
from json import load, loads, dumps
from iota_sdk import RoutesResponse, CongestionResponse, OutputWithMetadataResponse, ManaRewardsResponse, ValidatorsResponse, ValidatorResponse, InfoResponse, CommitteeResponse, IssuanceBlockHeaderResponse, Block, BlockWithMetadataResponse, OutputMetadata, OutputResponse, SlotCommitment, UtxoChangesResponse, UtxoChangesFullResponse


base_path = '../../sdk/tests/types/api/fixtures/'
T = TypeVar("T")


def test_api_responses():
    def test_api_response(cls_type: Generic[T], path: str):
        fixture_str = ''
        with open(base_path + path, "r", encoding="utf-8") as payload:
            fixture = load(payload)
        cls = cls_type.from_dict(fixture)

        # We need to sort the keys because optional fields in classes must be
        # last in Python
        fixture_str = dumps(fixture, sort_keys=True)
        recreated = dumps(
            loads(cls.to_json()), sort_keys=True)
        assert fixture_str == recreated

    # GET /api/routes
    test_api_response(RoutesResponse, "get-routes-response-example.json")
    # GET /api/core/v3/info
    test_api_response(InfoResponse, "get-info-response-example.json")
    # GET /api/core/v3/accounts/{bech32Address}/congestion
    test_api_response(CongestionResponse,
                      "get-congestion-estimate-response-example.json")
    # GET /api/core/v3/rewards/{outputId}
    test_api_response(ManaRewardsResponse, "get-mana-rewards-example.json")
    # GET /api/core/v3/validators
    test_api_response(ValidatorsResponse, "get-validators-example.json")
    # GET /api/core/v3/validators/{bech32Address}
    test_api_response(ValidatorResponse, "get-validator-example.json")
    # GET /api/core/v3/committee
    test_api_response(CommitteeResponse, "get-committee-example.json")
    # GET /api/core/v3/blocks/issuance
    test_api_response(IssuanceBlockHeaderResponse,
                      "get-buildingBlock-response-example.json")
    # GET /api/core/v3/blocks/{blockId}
    test_api_response(Block, "get-block-by-id-empty-response-example.json")
    test_api_response(Block, "tagged-data-block-example.json")
    test_api_response(Block, "transaction-block-example.json")
    test_api_response(
        Block, "get-block-by-id-validation-response-example.json")
    # GET /api/core/v3/blocks/{blockId}/metadata
    # TODO reenable when TIP is updated
    # test_api_response(BlockMetadataResponse,
    #                   "get-block-by-id-response-example-new-transaction.json")
    # test_api_response(BlockMetadataResponse,
    #                   "get-block-by-id-response-example-new.json")
    # test_api_response(BlockMetadataResponse,
    #                   "get-block-by-id-response-example-confirmed-transaction.json")
    # test_api_response(BlockMetadataResponse,
    #                   "get-block-by-id-response-example-confirmed.json")
    # test_api_response(BlockMetadataResponse,
    #                   "get-block-by-id-response-example-conflicting-transaction.json")
    # GET /api/core/v3/blocks/{blockId}/full
    test_api_response(BlockWithMetadataResponse,
                      "get-full-block-by-id-tagged-data-response-example.json")
    # GET /api/core/v3/outputs/{outputId}
    test_api_response(
        OutputResponse, "get-outputs-by-id-response-example.json")
    # GET /api/core/v3/outputs/{outputId}/metadata
    test_api_response(
        OutputMetadata, "get-output-metadata-by-id-response-unspent-example.json")
    test_api_response(
        OutputMetadata, "get-output-metadata-by-id-response-spent-example.json")
    # GET /api/core/v3/outputs/{outputId}/full
    test_api_response(OutputWithMetadataResponse,
                      "get-full-output-metadata-example.json")
    # GET /api/core/v3/transactions/{transactionId}/metadata
    # TODO reenable when TIP is updated
    # test_api_response(TransactionMetadataResponse,
    #                   "get-transaction-metadata-by-id-response-example.json")
    # GET /api/core/v3/commitments/{commitmentId}
    test_api_response(SlotCommitment, "get-commitment-response-example.json")
    # GET /api/core/v3/commitments/{commitmentId}/utxo-changes
    test_api_response(UtxoChangesResponse,
                      "get-utxo-changes-response-example.json")
    # GET /api/core/v3/commitments/{commitmentId}/utxo-changes/full
    test_api_response(UtxoChangesFullResponse,
                      "get-utxo-changes-full-response-example.json")
