# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example generates a new random mnemonic.

from iota_sdk import Utils

# Set the generated mnemonic as env variable MNEMONIC so it can be used in
# the next examples.
mnemonic = Utils.generate_mnemonic()
print(f'Mnemonic: {mnemonic}')
