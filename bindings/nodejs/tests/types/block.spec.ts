// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { expect, describe, it } from '@jest/globals';
import * as basic_block_tagged_data_payload_json from '../../../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json';
import * as basic_block_transaction_payload_json from '../../../../sdk/tests/types/fixtures/basic_block_transaction_payload.json';
import * as validation_block_json from '../../../../sdk/tests/types/fixtures/validation_block.json';
import * as protocol_parameters_json from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { Block, BlockId, parseBlock, ProtocolParameters } from '../../';

describe('Block tests', () => {
    it('compares basic block tagged data payload from a fixture', async () => {
        const block = parseBlock(basic_block_tagged_data_payload_json.block);
        expect(block).toBeInstanceOf(Block);
        const params: ProtocolParameters = JSON.parse(JSON.stringify(protocol_parameters_json.params));
        const expected_id: BlockId = basic_block_tagged_data_payload_json.id;
        expect(Block.id(block, params)).toEqual(expected_id);
    });

    it('compares basic block transaction payload from a fixture', async () => {
        const block = parseBlock(basic_block_transaction_payload_json.block);
        expect(block).toBeInstanceOf(Block);
        const params: ProtocolParameters = JSON.parse(JSON.stringify(protocol_parameters_json.params));
        const expected_id: BlockId = basic_block_transaction_payload_json.id;
        expect(Block.id(block, params)).toEqual(expected_id);
    });

    it('compares validation block from a fixture', async () => {
        const block = parseBlock(validation_block_json.block);
        expect(block).toBeInstanceOf(Block);
        const params: ProtocolParameters = JSON.parse(JSON.stringify(protocol_parameters_json.params));
        const expected_id: BlockId = validation_block_json.id;
        expect(Block.id(block, params)).toEqual(expected_id);
    });
});
