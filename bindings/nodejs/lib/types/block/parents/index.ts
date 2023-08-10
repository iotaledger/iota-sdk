// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { BlockId } from '../id';

type BoxedSlicePrefix<
    T,
    Min extends number,
    Max extends number,
    A extends (T | undefined)[] = [],
    O extends boolean = false,
> = O extends false
    ? Min extends A['length']
        ? BoxedSlicePrefix<T, Min, Max, A, true>
        : BoxedSlicePrefix<T, Min, Max, [...A, T], false>
    : Max extends A['length']
    ? A
    : BoxedSlicePrefix<T, Min, Max, [...A, T?], false>;

export type Parents<Min extends number, Max extends number> = BoxedSlicePrefix<
    BlockId,
    Min,
    Max
>;

// Array of Strongly referenced parents in the range of 1..8
export type StrongParents = Parents<1, 8>;
// Array of Weakly referenced parents in the range of 0..8
export type WeakParents = Parents<0, 8>;
// Array of Shallowly referenced parents in the range of 0..8
export type ShallowLikeParents = Parents<0, 8>;
