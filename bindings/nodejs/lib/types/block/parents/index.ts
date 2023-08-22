// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../id';

export type Parents = BlockId[];

/**
 *  Array of Strongly referenced parents.
 */
export type StrongParents = Parents;
/**
 *  Array of Weakly referenced parents.
 */
export type WeakParents = Parents;
/**
 *  Array of Shallowly referenced parents.
 */
export type ShallowLikeParents = Parents;
