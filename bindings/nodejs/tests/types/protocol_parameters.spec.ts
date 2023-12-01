// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { describe, it } from '@jest/globals';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { ProtocolParameters } from '../../lib/types/models/info/node-info-protocol';

describe('ProtocolParameters tests', () => {

    it('creates ProtocolParameters from a fixture', async () => {

        const params = protocol_parameters.params as unknown as ProtocolParameters;

        expect(params.type).toBe(0);
        expect(params.version).toBe(3);
        expect(params.networkName).toBe("TestJungle");
        expect(params.bech32Hrp).toBe("tgl");
    });
});
