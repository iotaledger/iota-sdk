# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import get_args
import pytest
from iota_sdk import BasicBlockBody, BlockBodyType, Block, Payload, PayloadType


def test_basic_block_with_tagged_data_payload():
    block_dict = {
        "type": 0,
        "strongParents": [
            "0x17c297a273facf4047e244a65eb34ee33b1f1698e1fff28679466fa2ad81c0e8",
            "0x9858e80fa0b37b6d9397e23d1f58ce53955a9be1aa8020c0d0e11672996c6db9"],
        "weakParents": [],
        "shallowLikeParents": [],
        "maxBurnedMana": "180500",
        "payload": {
            "type": 0,
            "tag": "0x484f524e4554205370616d6d6572",
            "data": "0x57652061726520616c6c206d616465206f662073746172647573742e0a436f756e743a20353436333730330a54696d657374616d703a20323032332d30372d31395430373a32323a32385a0a54697073656c656374696f6e3a20343732c2b573"}}
    block = BasicBlockBody.from_dict(block_dict)
    assert block.to_dict() == block_dict
    assert isinstance(block.payload, get_args(Payload))
    assert block.payload.type == PayloadType.TaggedData
    assert block.max_burned_mana == 180500

    block_to_dict = block.to_dict()
    # Make sure encoding is done correctly
    assert block_to_dict == block_dict


def test_block_with_tagged_data_payload():
    block_dict = {
        "protocolVersion": 3,
        "networkId": "10549460113735494767",
        "issuingTime": "1675563954966263210",
        "slotCommitmentId": "0x498bf08a5ed287bc87340341ffab28706768cd3a7035ae5e33932d9a12bb30940000000000000000",
        "latestFinalizedSlot": 21,
        "issuerId": "0x3370746f30705b7d0b42597459714d45241e5a64761b09627c447b751c7e145c",
        "block": {
            "type": 0,
            "strongParents": [
                "0x304442486c7a05361408585e4b5f7a67441c437528755a70041e0e557a6d4b2d7d4362083d492b57",
                "0x5f736978340a243d381b343b160b316a6b7d4b1e3c0355492e2e72113c2b126600157e69113c0b5c"
            ],
            "weakParents": [
                "0x0b5a48384f382f4a49471c4860683c6f0a0d446f012e1b117c4e405f5e24497c72691f43535c0b42"
            ],
            "shallowLikeParents": [
                "0x163007217803006078040b0f51507d3572355a457839095e572f125500401b7d220c772b56165a12"
            ],
            "maxBurnedMana": "180500",
            "payload": {
                "type": 0,
                "tag": "0x68656c6c6f20776f726c64",
                "data": "0x01020304"
            }
        },
        "signature": {
            "type": 0,
            "publicKey": "0x024b6f086177156350111d5e56227242034e596b7e3d0901180873740723193c",
            "signature": "0x7c274e5e771d5d60202d334f06773d3672484b1e4e6f03231b4e69305329267a4834374b0f2e0d5c6c2f7779620f4f534c773b1679400c52303d1f23121a4049"
        }
    }
    block = Block.from_dict(block_dict)
    assert block.to_dict() == block_dict
    assert isinstance(block.body, BasicBlockBody)
    assert block.body.type == BlockBodyType.Basic
    assert isinstance(block.body.payload, get_args(Payload))
    assert block.body.payload.type == PayloadType.TaggedData
    # TODO: determine the actual hash of the block
    # assert block.id() == "0x7ce5ad074d4162e57f83cfa01cd2303ef5356567027ce0bcee0c9f57bc11656e"


@pytest.mark.skip(reason="https://github.com/iotaledger/iota-sdk/issues/1387")
def test_basic_block_with_tx_payload():
    block_dict = {
        "type": 0,
        "strongParents": ["0x27532565d4c8cc886dfc6a2238e8d2a72369672bb1d1d762c33b72d41b0b07b8",
                          "0x604e6996bd1ec110642fec5b9c980d4b126eba5683e80a6e2cb905ded0cebd98",
                          "0x6a14368f99e875aee0e7078d9e2ec2ba6c4fff6a3cd63c73a9b2c296d4a8e697",
                          "0xc3f20eb06ce8be091579e2fbe6c109d108983fb0eff2c768e98c61e6fe71b4b7"],
        "weakParents": [],
        "shallowLikeParents": [],
        "maxBurnedMana": "180500",
        "payload": {"type": 1,
                    "transaction": {
                        "networkId": "1856588631910923207",
                        "inputs": [{"type": 0,
                                    "transactionId": "0xc6765035e75e319e9cd55ab16e7619f6cd658e7f421c71d9fe276c77fdf3f5b3",
                                    "transactionOutputIndex": 1}],
                        "outputs": [{"type": 3,
                                     "amount": "1000000",
                                     "unlockConditions": [{"type": 0,
                                                           "address": {"type": 0,
                                                                       "pubKeyHash": "0xa119005b26d46fc74cf9188b3cef8d01623e68146741ee698cabefd425dc01be"}}]},
                                    {"type": 3,
                                     "amount": "995000000",
                                     "unlockConditions": [{"type": 0,
                                                           "address": {"type": 0,
                                                                       "pubKeyHash": "0xa119005b26d46fc74cf9188b3cef8d01623e68146741ee698cabefd425dc01be"}}]}]},
                    "unlocks": [{"type": 0,
                                 "signature": {"type": 0,
                                               "publicKey": "0xa7af600976f440ec97d7bddbf17eacf0bfbf710e8cfb4ae3eae475d4ae8e1b16",
                                               "signature": "0x6bbe2eed95300a3d707af1bb17e04f83087fe31261256020fd00c24a54543c084079bed29c6d1479ee5acfd1e2fa32316e88c4c1577b4fbea3fe247f71114500"}}]}}
    block = BasicBlockBody.from_dict(block_dict)
    assert block.to_dict() == block_dict
    assert isinstance(block.payload, get_args(Payload))
    assert block.payload.type == PayloadType.SignedTransaction


@pytest.mark.skip(reason="https://github.com/iotaledger/iota-sdk/issues/1387")
def test_basic_block_with_tx_payload_all_output_types():
    block_dict = {
        "type": 0,
        "strongParents": [
            "0x053296e7434e8a4d602f8db30a5aaf16c01140212fe79d8132137cda1c38a60a", "0x559ec1d9a31c55bd27588ada2ade70fb5b13764ddd600e29c3b018761ba30e15", "0xe78e8cdbbeda89e3408eed51b77e0db5ba035f5f3bf79a8365435bba40697693", "0xee9d6e45dbc080694e6c827fecbc31ad9f654cf57404bc98f4cbca033f8e3139"], "weakParents": [], "shallowLikeParents": [], "payload": {
            "type": 1, "transaction": {
                "networkId": "1856588631910923207", "inputs": [
                    {
                        "type": 0, "transactionId": "0xa49f5a764c3fe22f702b5b238a75a648faae1863f61c14fac51ba58d26acb823", "transactionOutputIndex": 9}, {
                            "type": 0, "transactionId": "0x6f23b39ebe433f8b522d2e4360186cd3e6b21baf46c0a591c801161e505330b4", "transactionOutputIndex": 0}, {
                                "type": 0, "transactionId": "0x6f23b39ebe433f8b522d2e4360186cd3e6b21baf46c0a591c801161e505330b4", "transactionOutputIndex": 1}, {
                                    "type": 0, "transactionId": "0x6f23b39ebe433f8b522d2e4360186cd3e6b21baf46c0a591c801161e505330b4", "transactionOutputIndex": 2}], "inputsCommitment": "0xb6913235037feeeb74ea54ca0354bd7daee95e5a4fc65b67c960e5f0df6a339f", "outputs": [
                                        {
                                            "type": 4, "amount": "1000000", "accountId": "0xf90a577f1bae4587fdb00752a847b3a2a9d623743993e9e7abdd0440a004caee", "foundryCounter": 1, "unlockConditions": [
                                                {
                                                    "type": 4, "address": {
                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}, {
                                                            "type": 5, "address": {
                                                                "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}], "features": [
                                                                    {
                                                                        "type": 0, "address": {
                                                                            "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}, {
                                                                                "type": 2, "data": "0x010203"}], "immutableFeatures": [
                                                                                    {
                                                                                        "type": 1, "address": {
                                                                                            "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]}, {
                                                                                                "type": 5, "amount": "1000000", "serialNumber": 1, "tokenScheme": {
                                                                                                    "type": 0, "mintedTokens": "0x32", "meltedTokens": "0x0", "maximumSupply": "0x64"}, "unlockConditions": [
                                                                                                        {
                                                                                                            "type": 6, "address": {
                                                                                                                "type": 8, "accountId": "0xf90a577f1bae4587fdb00752a847b3a2a9d623743993e9e7abdd0440a004caee"}}]}, {
                                                                                                                    "type": 6, "amount": "1000000", "nftId": "0xbe01be2aa284eb07d1ec4ab8099c86c6cac38d8207d440dafa9560feeac49c62", "unlockConditions": [
                                                                                                                        {
                                                                                                                            "type": 0, "address": {
                                                                                                                                "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]}, {
                                                                                                                                    "type": 3, "amount": "1000000", "nativeTokens": [
                                                                                                                                        {
                                                                                                                                            "id": "0x08f90a577f1bae4587fdb00752a847b3a2a9d623743993e9e7abdd0440a004caee0100000000", "amount": "0x32"}], "unlockConditions": [
                                                                                                                                                {
                                                                                                                                                    "type": 0, "address": {
                                                                                                                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]}, {
                                                                                                                                                            "type": 3, "amount": "1000000", "unlockConditions": [
                                                                                                                                                                {
                                                                                                                                                                    "type": 0, "address": {
                                                                                                                                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]}, {
                                                                                                                                                                            "type": 3, "amount": "1000000", "unlockConditions": [
                                                                                                                                                                                {
                                                                                                                                                                                    "type": 0, "address": {
                                                                                                                                                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}], "features": [
                                                                                                                                                                                            {
                                                                                                                                                                                                "type": 2, "data": "0x0d25"}]}, {
                                                                                                                                                                                                    "type": 3, "amount": "234100", "unlockConditions": [
                                                                                                                                                                                                        {
                                                                                                                                                                                                            "type": 0, "address": {
                                                                                                                                                                                                                "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}, {
                                                                                                                                                                                                                    "type": 1, "returnAddress": {
                                                                                                                                                                                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}, "amount": "234000"}]}, {
                                                                                                                                                                                                                            "type": 3, "amount": "1000000", "unlockConditions": [
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    "type": 0, "address": {
                                                                                                                                                                                                                                        "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}, {
                                                                                                                                                                                                                                            "type": 3, "returnAddress": {
                                                                                                                                                                                                                                                "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}, "slotIndex": 1}]}, {
                                                                                                                                                                                                                                                    "type": 3, "amount": "1000000", "unlockConditions": [
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            "type": 0, "address": {
                                                                                                                                                                                                                                                                "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}, {
                                                                                                                                                                                                                                                                    "type": 2, "slotIndex": 1}]}, {
                                                                                                                                                                                                                                                                        "type": 3, "amount": "5578452198", "nativeTokens": [
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                "id": "0x080021bcfa2252a500348f73c939722d65c0354eab33b753ab09bc80a7f592c9a40100000000", "amount": "0x41"}, {
                                                                                                                                                                                                                                                                                    "id": "0x0808fb702d67fdb320b5959f152c0f962630515d904c71ed09447c341a6cc171de0100000000", "amount": "0x50"}, {
                                                                                                                                                                                                                                                                                        "id": "0x0808fb702d67fdb320b5959f152c0f962630515d904c71ed09447c341a6cc171de0200000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                            "id": "0x0808fb702d67fdb320b5959f152c0f962630515d904c71ed09447c341a6cc171de0300000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                "id": "0x080906d8ff0afdcb941cd3867186c8f23d5c889c7a1a8842a001871c8f76bcaf890100000000", "amount": "0xa"}, {
                                                                                                                                                                                                                                                                                                    "id": "0x08179dc4b298721f8bb60a591d5a00edc6e62ed941c133c4a4415b1ccc7d3804d90100000000", "amount": "0x42"}, {
                                                                                                                                                                                                                                                                                                        "id": "0x081d30f89a8655ce7514b5724ebae8c8f2160a223a6d8c91edca72de5e1477337b0100000000", "amount": "0x1a"}, {
                                                                                                                                                                                                                                                                                                            "id": "0x0822ceb3166ad125d310e6660f5fc292356f87f2f9566e982ea22154cec3847b3f0100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                "id": "0x0822ceb3166ad125d310e6660f5fc292356f87f2f9566e982ea22154cec3847b3f0200000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                    "id": "0x08261d3aa7f731a9ff784d6f239bfdc505bbe9902d8eace89c91c8e58429c200cb0100000000", "amount": "0x42"}, {
                                                                                                                                                                                                                                                                                                                        "id": "0x082a1d58d3d725f9d3af50699c2cfa022274b199a9f4060b2331bf059e285bd2730100000000", "amount": "0x313030"}, {
                                                                                                                                                                                                                                                                                                                            "id": "0x082ae3fdb7c757dbaae9d9463b63e7e3897f145c8c4a149bfe0ce4d316dc78f2500100000000", "amount": "0x1a"}, {
                                                                                                                                                                                                                                                                                                                                "id": "0x082ae3fdb7c757dbaae9d9463b63e7e3897f145c8c4a149bfe0ce4d316dc78f2500200000000", "amount": "0x3d2"}, {
                                                                                                                                                                                                                                                                                                                                    "id": "0x082cdf4c519401df914bf8ab3ebd1a1bb18da5babe7be82188f586a9c9a7bbdc160100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                        "id": "0x083156fbacf47e0b3ccaa5f4ffcbb9ae333fefcb4016261edfb302668eae242b050100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                            "id": "0x0833b41872b4ef228c10a99456fb08088a52a71f3ff23330287f6c8978bc5dd6df0100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                "id": "0x083637f9940377b8e461d66e09f73e61b4186dd63deca2ed518b8ea87c410492e80100000000", "amount": "0x3c"}, {
                                                                                                                                                                                                                                                                                                                                                    "id": "0x083c39ef7bd9a2eb640df6a36319a7fd51d4ca190ffd5d14572c9ebb54bdc6ecab0100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                        "id": "0x083c39ef7bd9a2eb640df6a36319a7fd51d4ca190ffd5d14572c9ebb54bdc6ecab0200000000", "amount": "0x45"}, {
                                                                                                                                                                                                                                                                                                                                                            "id": "0x08475073881df12705fc6f18f228385ac3d499d21e5e36333c78f3c7e124c4b1e60100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                "id": "0x084bd1dbdfecb771b4c56aa196cf90ca722ef489f5c12f2e11adb0fad4da8020060100000000", "amount": "0x5a"}, {
                                                                                                                                                                                                                                                                                                                                                                    "id": "0x084bd1dbdfecb771b4c56aa196cf90ca722ef489f5c12f2e11adb0fad4da8020060200000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                        "id": "0x084bd1dbdfecb771b4c56aa196cf90ca722ef489f5c12f2e11adb0fad4da8020060300000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                            "id": "0x0856ed1d7e6c86cf41c2978a700db8fe2686500f7d6e35f7ef15aecdb799833e5c0100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                "id": "0x085c6b799750bdf7e5a5c81144465a0676bc11dab74b997444ca369949341720e80100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x085fbfe07b06a54fda8ab7b6c72c49a34c4dcafaf1e5ba1f145fb06da1bba72a8d0100000000", "amount": "0xa"}, {
                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x086a62922fd743b541c987020d2cb2942cf789bcefe41572854119180cb8e037a90100000000", "amount": "0x46"}, {
                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x086f7011adb53642e8ed7db230c2307fe980f4aff2685c22f7c84a61ec558f691b0100000000", "amount": "0x3c"}, {
                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x086f7011adb53642e8ed7db230c2307fe980f4aff2685c22f7c84a61ec558f691b0200000000", "amount": "0x3de"}, {
                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x0871493f8559908cf5825e1a0334fa184f0e8b42136e472ec7e2e8127bc14202f70100000000", "amount": "0x46"}, {
                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x08722b1bf4f0295866c8bc75590d83b7422e47739e4b0048126fae45d0b5d330f90100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x08722b1bf4f0295866c8bc75590d83b7422e47739e4b0048126fae45d0b5d330f90200000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x089694cbf1a422b1262d3b34810b7b0a53f49b6b0856388f8121b5d681b23c38e10100000000", "amount": "0x14"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x089694cbf1a422b1262d3b34810b7b0a53f49b6b0856388f8121b5d681b23c38e10200000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x089694cbf1a422b1262d3b34810b7b0a53f49b6b0856388f8121b5d681b23c38e10300000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x089694cbf1a422b1262d3b34810b7b0a53f49b6b0856388f8121b5d681b23c38e10400000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x089786a7641d1268cb3b1cc7b514828f1511e3ae0b885835d284d3af85e1c3d3950100000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x0897e215b1e3ccb05c63842dc38db5007241cca966f341d674db2d9e886dc0ed410100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x089ad7373abf3a4dda4a8a2e64b112ac25dca24efcf51346f8b2d0212961234d0b0100000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x089c130fa264a23492f5876e4c2673154689fa8e30945c7b60c59050b20336d2b70100000000", "amount": "0x32"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x089c130fa264a23492f5876e4c2673154689fa8e30945c7b60c59050b20336d2b70200000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x089cfeccd4b71fc3d425755d972c1671346903dab3fda81ee54b807b752487d8250100000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x08aa2f74dc19d68bd3eb44f5b6648548b42b55f88c62374993301fd15c9ccf21270100000000", "amount": "0x32"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x08aa2f74dc19d68bd3eb44f5b6648548b42b55f88c62374993301fd15c9ccf21270200000000", "amount": "0x31303030"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x08ac83d1ce645b025a4a412f22e573973aadb50de1f8407d87b3cca4ed3e779a360100000000", "amount": "0x28"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x08d1bcfd507246eac6d93fee125e36e5eb8f62afc25bfff09785b6bcc560cf5dc00100000000", "amount": "0x3e8"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x08dc44610c24f32f26330440f3f0d4afb562a8dfd81afe7c2f79024f8f1b9e21940100000000", "amount": "0x63"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x08dda479a9d366af826f1a8e3f3290a6c230c39b8d2d1ba165bf737c71856e92640100000000", "amount": "0x14"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x08ea1cab7a1ba4ae5bdca654fdfe618fc92337030ecbae1d1f6b2b1fe4c6b569940200000000", "amount": "0x64"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x08ec6d781c5bdb7faebfa66dc529dc46e82a26fb90c5a5de07ee77d357d62529360100000000", "amount": "0x32"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x08f1802858831220b282ccc4c557676d61f79833869de378ce9a81f736976ce39f0100000000", "amount": "0x32"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            "id": "0x08f708a29e9619e847916de76c2e167e87a704c235dcbd7cda018865be7f561b5a0100000000", "amount": "0x4c"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "id": "0x08f708a29e9619e847916de76c2e167e87a704c235dcbd7cda018865be7f561b5a0200000000", "amount": "0x20"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    "id": "0x08fb64703098e00d81c5962f28d8504eae5998cf99ab4e37af0d3ea99180b2f6580100000000", "amount": "0x14"}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        "id": "0x08fe4090ca7623deffc5a570898d0844fe9f4763175af2c88a00958b26525b2b420100000000", "amount": "0x22"}], "unlockConditions": [
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "type": 0, "address": {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    "type": 0, "pubKeyHash": "0x7ffec9e1233204d9c6dce6812b1539ee96af691ca2e4d9065daa85907d33e5d3"}}]}]}, "unlocks": [
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            "type": 0, "signature": {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "type": 0, "publicKey": "0x67b7fc3f78763c9394fc4fcdb52cf3a973b6e064bdc3defb40a6cb2c880e6f5c", "signature": "0xc9ec7eba19c11b7a76f33a7781415a2f28fc3cf077fff4627f8c49604c77a5c6b4b4688a56bbe6e35a38bd97f1d03f5589050bd1f3372fc0ad57f8cb26f0da0e"}}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    "type": 1, "reference": 0}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        "type": 2, "reference": 1}, {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            "type": 1, "reference": 0}]}}
    block = BasicBlockBody.from_dict(block_dict)
    assert block.to_dict() == block_dict
    assert isinstance(block.payload, get_args(Payload))
    assert block.payload.type == PayloadType.SignedTransaction


@pytest.mark.skip(reason="https://github.com/iotaledger/iota-sdk/issues/1387")
def test_basic_block_with_tx_payload_with_tagged_data_payload():
    block_dict = {
        "type": 0,
        "strongParents": ["0x4bbba1f1fbfa58d8e65c018d0518da1c3ab57f05ffdd9c2e20565a99b42948df",
                          "0x9962a18f0161f6b883cb1e36b936684793867d97dc9ac226a929d8e434385e96",
                          "0xe532853c4a1e03e00a37c78a42afebf3570b1bb4a756c5ad651c0f0377548348",
                          "0xedbd8bd428bcff342de0656e368a881022dd353b51f272ed40c604c86915d97d"],
        "weakParents": [],
        "shallowLikeParents": [],
        "maxBurnedMana": "180500",
        "payload": {"type": 1,
                    "transaction": {
                        "networkId": "1856588631910923207",
                        "inputs": [{"type": 0,
                                    "transactionId": "0xeccfbdb73c0a4c9c0301b53a17e5aa301fbf0b079db9e88ff0e32e9e64214b28",
                                    "transactionOutputIndex": 5},
                                   {"type": 0,
                                    "transactionId": "0xf8052938858750c9c69b92b615a685fa2bb5833912b264142fc724e9510b0d0e",
                                    "transactionOutputIndex": 0}],
                        "inputsCommitment": "0x9702f2a625db14db2f67289828a9fdbe342477393572b9165b19964b2449061a",
                        "outputs": [{"type": 3,
                                     "amount": "1000000",
                                     "unlockConditions": [{"type": 0,
                                                           "address": {"type": 0,
                                                                       "pubKeyHash": "0x60200bad8137a704216e84f8f9acfe65b972d9f4155becb4815282b03cef99fe"}}]},
                                    {"type": 3,
                                     "amount": "50600",
                                     "unlockConditions": [{"type": 0,
                                                           "address": {"type": 0,
                                                                       "pubKeyHash": "0x74e8b1f10396eb5e8aeb16d666416802722436a88b5dd1a88e59c170b724c9cc"}}]}],
                        "payload": {"type": 5,
                                    "tag": "0x746167",
                                    "data": "0x64617461"}},
                    "unlocks": [{"type": 0,
                                 "signature": {"type": 0,
                                               "publicKey": "0x67b7fc3f78763c9394fc4fcdb52cf3a973b6e064bdc3defb40a6cb2c880e6f5c",
                                               "signature": "0x30cb012af3402be1b4b2ed18e2aba86839da06ba38ff3277c481e17c003f0199ba26f5613199e0d24035628bb2b69a6ea2a7682e41c30244996baf3a2adc1c00"}},
                                {"type": 1,
                                 "reference": 0}]}}
    block = BasicBlockBody.from_dict(block_dict)
    assert block.to_dict() == block_dict
    assert isinstance(block.payload, get_args(Payload))
    assert block.payload.type == PayloadType.SignedTransaction
