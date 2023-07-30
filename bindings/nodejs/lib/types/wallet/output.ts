// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Type } from 'class-transformer';
import { Address, AddressDiscriminator } from '../block/address';
import { Output, OutputDiscriminator, OutputId } from '../block/output';
import { IOutputMetadataResponse } from '../models/api';

/** Output to claim */
export enum OutputsToClaim {
    MicroTransactions = 'MicroTransactions',
    NativeTokens = 'NativeTokens',
    Nfts = 'Nfts',
    Amount = 'Amount',
    All = 'All',
}

/** An output with metadata */
export class OutputData {
    /** The identifier of an Output */
    outputId!: OutputId;
    /** The metadata of the output */
    metadata!: IOutputMetadataResponse;
    /** The actual Output */
    @Type(() => Output, {
        discriminator: OutputDiscriminator,
    })
    output!: Output;
    /** If an output is spent */
    isSpent!: boolean;
    /** Associated account address */
    @Type(() => Address, {
        discriminator: AddressDiscriminator,
    })
    address!: Address;
    /** Network ID */
    networkId!: string;
    /** Remainder */
    remainder!: boolean;
    /** BIP32 path */
    chain?: Segment[];
}

/** A Segment of the BIP32 path*/
export interface Segment {
    /** Whether the segment is hardened. */
    hardened: boolean;
    /** The bytes of the segment. */
    bs: Uint8Array;
}
