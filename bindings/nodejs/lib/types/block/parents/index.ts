// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from "../id";

type TupMinMax<
  T, Min extends number, Max extends number,
  A extends (T | undefined)[] = [], O extends boolean = false
  > = O extends false ? (
    Min extends A['length'] ? TupMinMax<T, Min, Max, A, true> : 
    TupMinMax<T, Min, Max, [...A, T], false>
  ) : Max extends A['length'] ? A : 
    TupMinMax<T, Min, Max, [...A, T?], false>;

export type Parents<Min extends number, Max extends number> = TupMinMax<BlockId, Min, Max>;

// Array of Strongly referenced parents in the range of 1..8
export type StrongParents = Parents<1, 8>;
// Array of Weakly referenced parents in the range of 0..8
export type WeakParents = Parents<0, 8>;
// Array of Shallowly referenced parents in the range of 0..8
export type ShallowLikeParents = Parents<0, 8>;

