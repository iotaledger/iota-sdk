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

/** An output with additional data */
export class OutputData {
    /** The output itself */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /** The metadata of the output */
    metadata!: OutputMetadataResponse;
    /** The output ID proof */
    OutputIdProof!: OutputIdProof;
    /** The corresponding output ID */
    outputId!: OutputId;
    /** The network ID the output belongs to */
    networkId!: string;
    /** Whether the output represents a remainder amount */
    remainder!: boolean;
}

/** A Segment of the BIP32 path*/
export interface Segment {
    /** Whether the segment is hardened. */
    hardened: boolean;
    /** The bytes of the segment. */
    bs: Uint8Array;
}
