// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { expect, describe, it } from '@jest/globals';
import * as basic_block_tagged_data_payload_json from '../../../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json';
import * as basic_block_transaction_payload_json from '../../../../sdk/tests/types/fixtures/basic_block_transaction_payload.json';
import * as validation_block_json from '../../../../sdk/tests/types/fixtures/validation_block.json';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { BasicBlockBody, Utils, ProtocolParameters, Block, HexEncodedString, TaggedDataPayload, Transaction, PayloadType, SignedTransactionPayload, ValidationBlockBody } from '../../';

describe('Block tests', () => {

    it('compares basic block tagged data payload from a fixture', async () => {
        const block = basic_block_tagged_data_payload_json.block as unknown as Block;
        const params = protocol_parameters.params as unknown as ProtocolParameters;
        const expected_id = basic_block_tagged_data_payload_json.id as unknown as HexEncodedString;
        // TODO: should we add an id() method on Block like we have in Python?
        expect(Utils.blockId(block, params)).toEqual(expected_id);
    });

    it('compares basic block transaction payload from a fixture', async () => {
        const block = basic_block_transaction_payload_json.block as unknown as Block;
        const params = protocol_parameters.params as unknown as ProtocolParameters;
        const expected_id = basic_block_transaction_payload_json.id as unknown as HexEncodedString;
        // TODO: should we add an id() method on Block like we have in Python?
        expect(Utils.blockId(block, params)).toEqual(expected_id);
    });

    it('compares validation block from a fixture', async () => {
        const block = validation_block_json.block as unknown as Block;
        const params = protocol_parameters.params as unknown as ProtocolParameters;
        const expected_id = validation_block_json.id as unknown as HexEncodedString;
        // TODO: should we add an id() method on Block like we have in Python?
        expect(Utils.blockId(block, params)).toEqual(expected_id);
    });
});
