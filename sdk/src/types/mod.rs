// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Common types required by nodes and clients APIs like blocks, responses and DTOs.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "block")]
pub mod block;
