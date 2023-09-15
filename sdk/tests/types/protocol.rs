// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::protocol::ProtocolParameters;

#[test]
fn params_serde_hash() {
    // Data from https://github.com/iotaledger/tips-draft/blob/tip49/tips/TIP-0049/tip-0049.md#protocol-parameter-example
    let protocol_params_json = serde_json::json!(
        {
            "type": 0,
            "version": 3,
            "networkName": "xxxNetwork",
            "bech32Hrp": "xxx",
            "rentStructure": {
              "vByteCost": 6,
              "vByteFactorData": 7,
              "vByteFactorKey": 8,
              "vByteFactorIssuerKeys": 9,
              "vByteFactorStakingFeature": 10,
              "vByteFactorDelegation": 10
            },
            "workScoreStructure": {
              "dataKilobyte": 1,
              "block": 2,
              "missingParent": 3,
              "input": 4,
              "contextInput": 5,
              "output": 6,
              "nativeToken": 7,
              "staking": 8,
              "blockIssuer": 9,
              "allotment": 10,
              "signatureEd25519": 11,
              "minStrongParentsThreshold": 12
            },
            "tokenSupply": "1234567890987654321",
            "genesisUnixTimestamp": "1681373293",
            "slotDurationInSeconds": 10,
            "slotsPerEpochExponent": 13,
            "manaStructure": {
                "manaBitsCount": 1,
                "manaGenerationRate": 1,
                "manaGenerationRateExponent": 27,
                "manaDecayFactors": [ 10, 20 ],
                "manaDecayFactorsExponent": 32,
                "manaDecayFactorEpochsSum": 1337,
                "manaDecayFactorEpochsSumExponent": 20
            },
            "stakingUnbondingPeriod": "11",
            "validationBlocksPerSlot": 10,
            "livenessThreshold": "3",
            "minCommittableAge": "10",
            "maxCommittableAge": "20",
            "epochNearingThreshold": "24",
            "congestionControlParameters": {
              "rmcMin": "500",
              "increase": "500",
              "decrease": "500",
              "increaseThreshold": 800000,
              "decreaseThreshold": 500000,
              "schedulerRate": 100000,
              "minMana": "1",
              "maxBufferSize": 3276800
            },
            "versionSignaling": {
              "windowSize": 3,
              "windowTargetRatio": 4,
              "activationOffset": 1
            }
          }
    );
    let protocol_params = serde_json::from_value::<ProtocolParameters>(protocol_params_json).unwrap();
    let hash = protocol_params.hash();

    assert_eq!(
        hash.to_string(),
        "0xd379bdceb68aa77dada50ae7e3493b8f0b6ed28d26813620ce893afad541eb29"
    );
}
