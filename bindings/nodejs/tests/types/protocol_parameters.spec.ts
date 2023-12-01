// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { describe, it } from '@jest/globals';
import { expect } from '@jest/globals';
import { ProtocolParameters } from '../../lib/types/models/info/node-info-protocol';
import { Utils } from '../../lib/utils/utils';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';

describe('ProtocolParameters tests', () => {

    it('compares ProtocolParameters hash from a fixture', async () => {

        const params = protocol_parameters.params as unknown as ProtocolParameters;
        const hash = Utils.protocolParametersHash(params);

        const expected_hash = protocol_parameters.hash;
        
        expect(hash).toEqual(expected_hash);
    });
});
