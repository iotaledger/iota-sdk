// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance, Type } from 'class-transformer';
import { describe, it } from '@jest/globals';
import { expect } from '@jest/globals';
import * as protocol_parameters from '../../../../sdk/tests/types/fixtures/protocol_parameters.json';
import { ProtocolParameters, WorkScoreParameters, ManaParameters } from '../../lib/types/models/info/node-info-protocol';
import { StorageScoreParameters } from '../../lib/types/models';

describe('ProtocolParameters tests', () => {

    it('creates ProtocolParameters from a fixture', async () => {

        // TODO: is there a way around `as unknown`? The string->bigint parameters are the issue.
        const params = protocol_parameters.params as unknown as ProtocolParameters;

        // Check whether parameters were correctly set
        expect(params.type).toEqual(0);
        expect(params.version).toEqual(3);
        expect(params.networkName).toEqual("TestJungle");
        expect(params.bech32Hrp).toEqual("tgl");

        // TODO: is there a way around `as unknown`? The string->bigint parameters are the issue.
        const storageScoreParams = protocol_parameters.params.storageScoreParameters as unknown as StorageScoreParameters;

        expect(storageScoreParams.storageCost).toEqual("0");
        expect(storageScoreParams.factorData).toEqual(0);
        expect(storageScoreParams.offsetOutputOverhead).toEqual("0");
        expect(storageScoreParams.offsetEd25519BlockIssuerKey).toEqual("0");
        expect(storageScoreParams.offsetStakingFeature).toEqual("0");
        expect(storageScoreParams.offsetDelegation).toEqual("0");

        const workScoreParameters = protocol_parameters.params.workScoreParameters as WorkScoreParameters;
        const manaParameters = protocol_parameters.params.manaParameters as ManaParameters;
    });
});
