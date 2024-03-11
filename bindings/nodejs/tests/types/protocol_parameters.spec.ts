// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import 'reflect-metadata';

import { expect, describe, it } from '@jest/globals';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { ProtocolParameters } from '../../lib/types/models/api/node/info-response';
import { Utils } from '../../';

describe('ProtocolParameters tests', () => {

    it('compares ProtocolParameters hash from a fixture', async () => {

        const params: ProtocolParameters = JSON.parse(JSON.stringify(protocol_parameters.params));
        const hash = Utils.protocolParametersHash(params);
        const expected_hash = protocol_parameters.hash;

        expect(hash).toEqual(expected_hash);
    });
});
