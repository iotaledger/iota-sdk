// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg))]
// TODO missing_docs, unreachable_pub
#![deny(clippy::nursery, rust_2018_idioms, warnings)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::significant_drop_in_scrutinee
)]

#[macro_use]
extern crate serde;
extern crate alloc;

pub mod client;
pub mod pow;
pub mod types;
pub mod wallet;
