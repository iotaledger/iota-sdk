// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Constants for the Client

use std::time::Duration;

use crate::types::block::address::Hrp;

/// Default timeout for all API requests
pub(crate) const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(15);
/// Interval in which the node info will be requested and healthy nodes will be added to the healthy node pool
pub(crate) const NODE_SYNC_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_MIN_QUORUM_SIZE: usize = 3;
pub(crate) const DEFAULT_QUORUM_THRESHOLD: usize = 66;
pub(crate) const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
#[cfg(not(target_family = "wasm"))]
pub(crate) const MAX_PARALLEL_API_REQUESTS: usize = 100;
/// Max allowed difference between local time and tangle time, 5 minutes in seconds
pub(crate) const FIVE_MINUTES_IN_NANOSECONDS: u64 = 300_000_000_000;
/// Delay for caching a node info response in WASM runtime
#[cfg(target_family = "wasm")]
pub(crate) const CACHE_NETWORK_INFO_TIMEOUT_IN_SECONDS: u32 = 60;

/// Bech32 hrp for the IOTA mainnet <https://github.com/satoshilabs/slips/blob/master/slip-0173.md>
pub const IOTA_BECH32_HRP: Hrp = Hrp::from_str_unchecked("iota");
/// Bech32 hrp for the IOTA testnet <https://github.com/satoshilabs/slips/blob/master/slip-0173.md>
pub const IOTA_TESTNET_BECH32_HRP: Hrp = Hrp::from_str_unchecked("atoi");
/// Bech32 hrp for the Shimmer mainnet <https://github.com/satoshilabs/slips/blob/master/slip-0173.md>
pub const SHIMMER_BECH32_HRP: Hrp = Hrp::from_str_unchecked("smr");
/// Bech32 hrp for the Shimmer testnet <https://github.com/satoshilabs/slips/blob/master/slip-0173.md>
pub const SHIMMER_TESTNET_BECH32_HRP: Hrp = Hrp::from_str_unchecked("rms");

/// IOTA coin type <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>
pub const IOTA_COIN_TYPE: u32 = 4218;
/// Shimmer coin type <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>
pub const SHIMMER_COIN_TYPE: u32 = 4219;
/// Ethereum coin type <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>
pub const ETHER_COIN_TYPE: u32 = 60;
