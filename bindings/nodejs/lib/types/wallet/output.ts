// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Output, OutputDiscriminator, OutputId } from '../block/output';
import { OutputIdProof, OutputMetadataResponse } from '../models/api';

/** Output to claim */
export enum OutputsToClaim {
    MicroTransactions = 'MicroTransactions',
    NativeTokens = 'NativeTokens',
    Nfts = 'Nfts',
    Amount = 'Amount',
    All = 'All',
}

/** An output with extended metadata */
export class OutputWithExtendedMetadata {
    /** The output object itself */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /** The metadata of the output */
    metadata!: OutputMetadataResponse;
    /** The identifier of an Output */
    outputId!: OutputId;
    /** The output ID proof */
    OutputIdProof!: OutputIdProof;
    /** Network ID */
    networkId!: string;
    /** Remainder */
    remainder!: boolean;
}

/** A Segment of the BIP32 path*/
export interface Segment {
    /** Whether the segment is hardened. */
    hardened: boolean;
    /** The bytes of the segment. */
    bs: Uint8Array;
}
