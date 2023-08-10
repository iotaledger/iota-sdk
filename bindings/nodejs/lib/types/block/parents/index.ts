// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../id';

export type Parents = BlockId[];

/**
 *  Array of Strongly referenced parents in the range of 1..8
 */
export type StrongParents = Parents;
/**
 *  Array of Weakly referenced parents in the range of 0..8
 */
export type WeakParents = Parents;
/**
 *  Array of Shallowly referenced parents in the range of 0..8
 */
export type ShallowLikeParents = Parents;
