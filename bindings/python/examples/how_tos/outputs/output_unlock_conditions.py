from iota_sdk import *
from dotenv import load_dotenv
import json

load_dotenv()

# Create a Client instance
client = Client()

hex_address = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

alias_hex_address = Utils.bech32_to_hex(
    'rms1pr59qm43mjtvhcajfmupqf23x29llam88yecn6pyul80rx099krmv2fnnux')

address_unlock_condition = AddressUnlockCondition(
    Ed25519Address(hex_address)
)

token_scheme = TokenScheme(0, 50, 100)

# Most simple output
basic_output = client.build_basic_output(
    unlock_conditions=[address_unlock_condition],
)
outputs = [basic_output]

# Output with storage deposit return
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        StorageDepositReturnUnlockCondition(
            1000000,
            Ed25519Address(hex_address),
        ),
    ],
)
outputs.append(basic_output)

# Output with timelock
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        TimelockUnlockCondition(1),
    ],
)
outputs.append(basic_output)

# Output with expiration
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        ExpirationUnlockCondition(
            1,
            Ed25519Address(hex_address),
        ),
    ],
)
outputs.append(basic_output)

# Output with governor and state controller unlock condition
alias_output = client.build_alias_output(
    alias_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        GovernorAddressUnlockCondition(
            Ed25519Address(hex_address),
        ),
        StateControllerAddressUnlockCondition(
            Ed25519Address(hex_address),
        ),
    ],
)
outputs.append(alias_output)

# Output with immutable alias unlock condition
foundry_output = client.build_foundry_output(
    serial_number=1,
    token_scheme=token_scheme,
    unlock_conditions=[
        ImmutableAliasAddressUnlockCondition(
            AliasAddress(alias_hex_address),
        ),
    ],
)
outputs.append(foundry_output)

print(json.dumps(outputs, indent=2))
