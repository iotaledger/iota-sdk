// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::protocol::ProtocolParameters;
use packable::PackableExt;

// Test from https://github.com/iotaledger/tips-draft/blob/tip49/tips/TIP-0049/tip-0049.md#protocol-parameter-example
#[test]
fn params_serde_hash() {
    let protocol_params_json = serde_json::json!(
        {
            "type": 0,
            "version": 3,
            "networkName": "TestJungle",
            "bech32Hrp": "tgl",
            "rentParameters": {
              "storageCost": "0",
              "factorData": 0,
              "offsetOutputOverhead": "0",
              "offsetEd25519BlockIssuerKey": "0",
              "offsetStakingFeature": "0",
              "offsetDelegation": "0"
            },
            "workScoreParameters": {
              "dataByte": 0,
              "block": 1,
              "input": 0,
              "contextInput": 0,
              "output": 0,
              "nativeToken": 0,
              "staking": 0,
              "blockIssuer": 0,
              "allotment": 0,
              "signatureEd25519": 0
            },
            "tokenSupply": "2779530283277761",
            "genesisUnixTimestamp": "1695275822",
            "slotDurationInSeconds": 10,
            "slotsPerEpochExponent": 13,
            "manaParameters": {
              "bitsCount": 63,
              "generationRate": 1,
              "generationRateExponent": 17,
              "decayFactors": [
                10,
                20
              ],
              "decayFactorsExponent": 32,
              "decayFactorEpochsSum": 2420916375_u32,
              "decayFactorEpochsSumExponent": 21
            },
            "stakingUnbondingPeriod": 10,
            "validationBlocksPerSlot": 10,
            "punishmentEpochs": 10,
            "livenessThresholdLowerBound": 15,
            "livenessThresholdUpperBound": 30,
            "minCommittableAge": 10,
            "maxCommittableAge": 20,
            "epochNearingThreshold": 24,
            "congestionControlParameters": {
              "minReferenceManaCost": "1",
              "increase": "0",
              "decrease": "0",
              "increaseThreshold": 800000,
              "decreaseThreshold": 500000,
              "schedulerRate": 100000,
              "maxBufferSize": 1000,
              "maxValidationBufferSize": 100
            },
            "versionSignaling": {
              "windowSize": 7,
              "windowTargetRatio": 5,
              "activationOffset": 7
            },
            "rewardsParameters": {
              "profitMarginExponent": 8,
              "bootstrappingDuration": 1154,
              "manaShareCoefficient": "2",
              "decayBalancingConstantExponent": 8,
              "decayBalancingConstant": "1",
              "poolCoefficientExponent": 31
            }
        }

    );
    let protocol_params = serde_json::from_value::<ProtocolParameters>(protocol_params_json).unwrap();
    let protocol_params_bytes = protocol_params.pack_to_vec();

    #[rustfmt::skip]
    assert_eq!(
        protocol_params_bytes,
        [0, 3, 10, 84, 101, 115, 116, 74, 117, 110, 103, 108, 101, 3, 116, 103, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 93, 45, 211, 247, 223, 9, 0, 46, 219, 11, 101, 0, 0, 0, 0, 10, 13, 63, 1, 17, 2, 0, 10, 0, 0, 0, 20, 0, 0, 0, 32, 151, 64, 76, 144, 21, 10, 0, 0, 0, 10, 10, 0, 0, 0, 15, 0, 30, 0, 10, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 12, 0, 32, 161, 7, 0, 160, 134, 1, 0, 232, 3, 0, 0, 100, 0, 0, 0, 7, 5, 7, 8, 130, 4, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 8, 1, 0, 0, 0, 0, 0, 0, 0, 31]
    );

    let hash = protocol_params.hash().to_string();
    assert_eq!(
        hash,
        "0x0c00425134785bf2dbe42e4ec7e288009ebdc38ced797beaa45d5213092021cb"
    );
}
