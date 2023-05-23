# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr


def utf8_to_hex(utf8_data: str) -> HexStr:
    return HexStr('0x'+utf8_data.encode('utf-8').hex())


def hex_to_utf8(hex_data: HexStr) -> str:
    return bytes.fromhex(hex_data[2:]).decode('utf-8')
