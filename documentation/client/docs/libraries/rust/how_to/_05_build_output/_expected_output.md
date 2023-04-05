```plaintext
[
    BasicOutput {
        amount: 1000000,
        native_tokens: NativeTokens(
            [],
        ),
        unlock_conditions: UnlockConditions(
            [
                AddressUnlockCondition(
                    Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                ),
            ],
        ),
        features: Features(
            [],
        ),
    },
    BasicOutput {
        amount: 1000000,
        native_tokens: NativeTokens(
            [],
        ),
        unlock_conditions: UnlockConditions(
            [
                AddressUnlockCondition(
                    Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                ),
            ],
        ),
        features: Features(
            [
                MetadataFeature(0x48656c6c6f2c20576f726c6421),
            ],
        ),
    },
    BasicOutput {
        amount: 1000000,
        native_tokens: NativeTokens(
            [],
        ),
        unlock_conditions: UnlockConditions(
            [
                AddressUnlockCondition(
                    Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                ),
                StorageDepositReturnUnlockCondition {
                    return_address: Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                    amount: 1000000,
                },
            ],
        ),
        features: Features(
            [],
        ),
    },
    BasicOutput {
        amount: 1000000,
        native_tokens: NativeTokens(
            [],
        ),
        unlock_conditions: UnlockConditions(
            [
                AddressUnlockCondition(
                    Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                ),
                ExpirationUnlockCondition {
                    return_address: Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                    timestamp: 1,
                },
            ],
        ),
        features: Features(
            [],
        ),
    },
    BasicOutput {
        amount: 1000000,
        native_tokens: NativeTokens(
            [],
        ),
        unlock_conditions: UnlockConditions(
            [
                AddressUnlockCondition(
                    Ed25519Address(0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3),
                ),
                TimelockUnlockCondition(
                    1,
                ),
            ],
        ),
        features: Features(
            [],
        ),
    },
]
```