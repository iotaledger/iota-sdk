// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../id';

export type Parents = BlockId[];

/**
 *  Array of strongly referenced parents.
 */
export type StrongParents = Parents;
/**
 *  Array of weakly referenced parents.
 */
export type WeakParents = Parents;
/**
 *  Array of shallowly referenced parents.
 */
export type ShallowLikeParents = Parents;
