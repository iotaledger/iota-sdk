// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::protocol::ProtocolParameters;
use packable::PackableExt;

#[test]
fn params_serde_hash() {
    // Test from https://github.com/iotaledger/tips-draft/blob/tip49/tips/TIP-0049/tip-0049.md#protocol-parameter-example
    let protocol_params_json = serde_json::json!(
      {
        "type":0,
        "version":3,
        "networkName":"xxxNetwork",
        "bech32Hrp":"xxx",
        "rentStructure": {
          "vByteCost":6,
          "vByteFactorData":7,
          "vByteFactorKey":8,
          "vByteFactorBlockIssuerKey":9,
          "vByteFactorStakingFeature":10,
          "vByteFactorDelegation":10
        },
        "workScoreStructure":{
          "workScoreDataKilobyte":1,
          "workScoreBlock":2,
          "workScoreMissingParent":3,
          "workScoreInput":4,
          "workScoreContextInput":5,
          "workScoreOutput":6,
          "workScoreNativeToken":7,
          "workScoreStaking":8,
          "workScoreBlockIssuer":9,
          "workScoreAllotment":10,
          "workScoreSignatureEd25519":11,
          "workScoreMinStrongParentsThreshold":12
        },
        "tokenSupply":"1234567890987654321",
        "genesisUnixTimestamp":"1681373293",
        "slotDurationInSeconds":10,
        "slotsPerEpochExponent":13,
        "manaStructure": {
          "bitsCount":1,
          "generationRate":1,
          "generationRateExponent":27,
          "decayFactors":[10,20],
          "decayFactorsExponent":32,
          "decayFactorEpochsSum":1337,
          "decayFactorEpochsSumExponent":20
        },
        "stakingUnbondingPeriod":"11",
        "validationBlocksPerSlot":10,
        "punishmentEpochs":"9",
        "livenessThreshold":"3",
        "minCommittableAge":"10",
        "maxCommittableAge":"20",
        "epochNearingThreshold":"24",
        "congestionControlParameters": {
          "minReferenceManaCost":"500",
          "increase":"500",
          "decrease":"500",
          "increaseThreshold":800000,
          "decreaseThreshold":500000,
          "schedulerRate":100000,
          "minMana":"1",
          "maxBufferSize":1000,
          "maxValidationBufferSize":100
        },
        "versionSignaling": {
          "windowSize":3,
          "windowTargetRatio":4,
          "activationOffset":1
        }
      }
    );
    let protocol_params = serde_json::from_value::<ProtocolParameters>(protocol_params_json).unwrap();
    let protocol_params_bytes = protocol_params.pack_to_vec();

    assert_eq!(
        protocol_params_bytes,
        [
            0, 3, 10, 120, 120, 120, 78, 101, 116, 119, 111, 114, 107, 3, 120, 120, 120, 6, 0, 0, 0, 7, 8, 9, 10, 10,
            1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0,
            10, 0, 0, 0, 11, 0, 0, 0, 12, 177, 28, 108, 177, 244, 16, 34, 17, 109, 184, 55, 100, 0, 0, 0, 0, 10, 13, 1,
            1, 27, 2, 0, 10, 0, 0, 0, 20, 0, 0, 0, 32, 57, 5, 0, 0, 20, 11, 0, 0, 0, 0, 0, 0, 0, 10, 0, 9, 0, 0, 0, 0,
            0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0,
            244, 1, 0, 0, 0, 0, 0, 0, 244, 1, 0, 0, 0, 0, 0, 0, 244, 1, 0, 0, 0, 0, 0, 0, 0, 53, 12, 0, 32, 161, 7, 0,
            160, 134, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 232, 3, 0, 0, 100, 0, 0, 0, 3, 4, 1
        ]
    );

    let hash = protocol_params.hash();

    assert_eq!(
        hash.to_string(),
        "0x9d3e39699e38db1d6e6777a40f82b1b5030e645cffb08dba3a157105bd6bfac8"
    );
}
