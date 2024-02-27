// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::OnceLock;

use crate::types::block::{
    address::Hrp,
    mana::{ManaParameters, RewardsParameters},
    output::StorageScoreParameters,
    protocol::{CongestionControlParameters, ProtocolParameters, VersionSignalingParameters, WorkScoreParameters},
    PROTOCOL_VERSION,
};

/// Returns IOTA mainnet [`ProtocolParameters`] for testing purposes.
pub fn iota_mainnet_protocol_parameters() -> &'static ProtocolParameters {
    static PARAMS: OnceLock<ProtocolParameters> = OnceLock::new();
    PARAMS.get_or_init(|| {
        ProtocolParameters {
            kind: 0,
            version: PROTOCOL_VERSION,
            network_name: String::from("iota-mainnet").try_into().unwrap(),
            bech32_hrp: Hrp::from_str_unchecked("iota"),
            storage_score_parameters: StorageScoreParameters {
                storage_cost: 100,
                factor_data: 1,
                offset_output_overhead: 10,
                offset_ed25519_block_issuer_key: 100,
                offset_staking_feature: 100,
                offset_delegation: 100,
            },
            work_score_parameters: WorkScoreParameters {
                data_byte: 1,
                block: 1500,
                input: 10,
                context_input: 20,
                output: 20,
                native_token: 20,
                staking: 5000,
                block_issuer: 1000,
                allotment: 1000,
                signature_ed25519: 1000,
            },
            token_supply: 1_813_620_509_061_365,
            genesis_slot: 0,
            #[cfg(not(target_family = "wasm"))]
            genesis_unix_timestamp: time::OffsetDateTime::now_utc().unix_timestamp() as _,
            #[cfg(target_family = "wasm")]
            genesis_unix_timestamp: instant::SystemTime::now()
                .duration_since(instant::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            slot_duration_in_seconds: 10,
            epoch_nearing_threshold: 60,
            slots_per_epoch_exponent: 13,
            mana_parameters: ManaParameters {
                bits_count: 63,
                generation_rate: 1,
                generation_rate_exponent: 17,
                // Derived
                decay_factors: Default::default(),
                decay_factors_exponent: 32,
                // Derived
                decay_factor_epochs_sum: Default::default(),
                decay_factor_epochs_sum_exponent: 21,
                annual_decay_factor_percentage: 70,
            },
            staking_unbonding_period: 10,
            validation_blocks_per_slot: 10,
            punishment_epochs: 10,
            liveness_threshold_lower_bound: 15,
            liveness_threshold_upper_bound: 30,
            min_committable_age: 10,
            max_committable_age: 20,
            congestion_control_parameters: CongestionControlParameters {
                min_reference_mana_cost: 1,
                increase: 10,
                decrease: 10,
                increase_threshold: 800000,
                decrease_threshold: 500000,
                scheduler_rate: 100000,
                max_buffer_size: 1000,
                max_validation_buffer_size: 100,
            },
            version_signaling_parameters: VersionSignalingParameters {
                window_size: 7,
                window_target_ratio: 5,
                activation_offset: 7,
            },
            rewards_parameters: RewardsParameters {
                profit_margin_exponent: 8,
                // Derived
                bootstrapping_duration: Default::default(),
                reward_to_generation_ratio: 2,
                // Derived
                initial_target_rewards_rate: Default::default(),
                // Derived
                final_target_rewards_rate: Default::default(),
                pool_coefficient_exponent: 11,
                retention_period: 384,
            },
            target_committee_size: 32,
            chain_switching_threshold: 3,
        }
        .with_derived_values()
    })
}

/// Returns Shimmer mainnet [`ProtocolParameters`] for testing purposes.
pub fn shimmer_mainnet_protocol_parameters() -> &'static ProtocolParameters {
    static PARAMS: OnceLock<ProtocolParameters> = OnceLock::new();
    PARAMS.get_or_init(|| {
        ProtocolParameters {
            kind: 0,
            version: PROTOCOL_VERSION,
            network_name: String::from("shimmer-mainnet").try_into().unwrap(),
            bech32_hrp: Hrp::from_str_unchecked("smr"),
            storage_score_parameters: StorageScoreParameters {
                storage_cost: 100,
                factor_data: 1,
                offset_output_overhead: 10,
                offset_ed25519_block_issuer_key: 100,
                offset_staking_feature: 100,
                offset_delegation: 100,
            },
            work_score_parameters: WorkScoreParameters {
                data_byte: 0,
                block: 1,
                input: 0,
                context_input: 0,
                output: 0,
                native_token: 0,
                staking: 0,
                block_issuer: 0,
                allotment: 0,
                signature_ed25519: 0,
            },
            token_supply: 1_813_620_509_061_365,
            genesis_slot: 0,
            #[cfg(not(target_family = "wasm"))]
            genesis_unix_timestamp: time::OffsetDateTime::now_utc().unix_timestamp() as _,
            #[cfg(target_family = "wasm")]
            genesis_unix_timestamp: instant::SystemTime::now()
                .duration_since(instant::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            slot_duration_in_seconds: 10,
            epoch_nearing_threshold: 60,
            slots_per_epoch_exponent: 13,
            mana_parameters: ManaParameters {
                bits_count: 63,
                generation_rate: 1,
                generation_rate_exponent: 17,
                // Derived
                decay_factors: Default::default(),
                decay_factors_exponent: 32,
                // Derived
                decay_factor_epochs_sum: Default::default(),
                decay_factor_epochs_sum_exponent: 21,
                annual_decay_factor_percentage: 70,
            },
            staking_unbonding_period: 10,
            validation_blocks_per_slot: 10,
            punishment_epochs: 10,
            liveness_threshold_lower_bound: 15,
            liveness_threshold_upper_bound: 30,
            min_committable_age: 10,
            max_committable_age: 20,
            congestion_control_parameters: CongestionControlParameters {
                min_reference_mana_cost: 1,
                increase: 10,
                decrease: 10,
                increase_threshold: 800000,
                decrease_threshold: 500000,
                scheduler_rate: 100000,
                max_buffer_size: 1000,
                max_validation_buffer_size: 100,
            },
            version_signaling_parameters: VersionSignalingParameters {
                window_size: 7,
                window_target_ratio: 5,
                activation_offset: 7,
            },
            rewards_parameters: RewardsParameters {
                profit_margin_exponent: 8,
                // Derived
                bootstrapping_duration: Default::default(),
                reward_to_generation_ratio: 2,
                // Derived
                initial_target_rewards_rate: Default::default(),
                // Derived
                final_target_rewards_rate: Default::default(),
                pool_coefficient_exponent: 11,
                retention_period: 384,
            },
            target_committee_size: 32,
            chain_switching_threshold: 3,
        }
        .with_derived_values()
    })
}

impl ProtocolParameters {
    pub(crate) fn with_derived_values(mut self) -> Self {
        self.derive_mana_decay_factors();
        self.derive_mana_decay_factors_epochs_sum();
        self.derive_bootstrapping_duration();
        self.derive_target_rewards_rates();
        self
    }

    pub(crate) fn derive_mana_decay_factors(&mut self) {
        self.mana_parameters.decay_factors = {
            let epochs_in_table = (u16::MAX as usize).min(self.epochs_per_year().floor() as usize);
            let decay_per_epoch = self.decay_per_epoch();
            (1..=epochs_in_table)
                .map(|epoch| {
                    (decay_per_epoch.powi(epoch as _) * 2f64.powi(self.mana_parameters().decay_factors_exponent() as _))
                        .floor() as u32
                })
                .collect::<Box<[_]>>()
        }
        .try_into()
        .unwrap();
    }

    pub(crate) fn derive_mana_decay_factors_epochs_sum(&mut self) {
        self.mana_parameters.decay_factor_epochs_sum = {
            let delta = self.epochs_per_year().recip();
            let annual_decay_factor = self.mana_parameters().annual_decay_factor();
            (annual_decay_factor.powf(delta) / (1.0 - annual_decay_factor.powf(delta))
                * (2f64.powi(self.mana_parameters().decay_factor_epochs_sum_exponent() as _)))
            .floor() as _
        };
    }

    pub(crate) fn derive_bootstrapping_duration(&mut self) {
        self.rewards_parameters.bootstrapping_duration =
            (self.epochs_per_year() / -self.mana_parameters().annual_decay_factor().ln()).floor() as _;
    }

    pub(crate) fn derive_target_rewards_rates(&mut self) {
        self.rewards_parameters.final_target_rewards_rate = (self.token_supply()
            * self.rewards_parameters().reward_to_generation_ratio() as u64
            * self.mana_parameters().generation_rate() as u64)
            >> (self.mana_parameters().generation_rate_exponent() - self.slots_per_epoch_exponent());
        let bootstrapping_duration_years =
            self.rewards_parameters().bootstrapping_duration() as f64 * self.epochs_per_year().exp();
        self.rewards_parameters.initial_target_rewards_rate = (self.rewards_parameters.final_target_rewards_rate as f64
            * (self.mana_parameters().annual_decay_factor() * bootstrapping_duration_years).exp())
        .floor() as _;
    }
}
