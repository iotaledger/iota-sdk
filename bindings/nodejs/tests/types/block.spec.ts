// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { expect, describe, it } from '@jest/globals';
import * as basic_block_tagged_data_payload from '../../../../sdk/tests/types/fixtures/basic_block_tagged_data_payload.json';
import * as basic_block_transacton_payload from '../../../../sdk/tests/types/fixtures/basic_block_transaction_payload.json';
import * as validation_block from '../../../../sdk/tests/types/fixtures/validation_block.json';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { BasicBlockBody, Utils, ProtocolParameters, Block, HexEncodedString, TaggedDataPayload } from '../../';

describe('Block tests', () => {

    it('compares basic block tagged data payload from a fixture', async () => {
        const params = protocol_parameters.params as unknown as ProtocolParameters;
        const block = basic_block_tagged_data_payload.block as unknown as Block;
        
        expect(block.isBasic).toBe(true);
        expect(block.body.isBasic).toBe(true);
        expect(block.asBasic).toBeInstanceOf(BasicBlockBody);

        const basic_block = block.body.asBasic();
        expect(basic_block.payload).toBeInstanceOf(TaggedDataPayload);
        expect(basic_block.maxBurnedMana).toEqual(864);

        const expected_id = basic_block_tagged_data_payload.id as unknown as HexEncodedString;
        // TODO: should we add an id() method on Block like we have in Python?
        const id = Utils.blockId(block, params);
        expect(id).toEqual(expected_id);
    });
});
