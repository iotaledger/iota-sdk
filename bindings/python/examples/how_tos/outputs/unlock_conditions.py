import json

from dotenv import load_dotenv
from iota_sdk import (
    AddressUnlockCondition,
    Client,
    Utils,
    ExpirationUnlockCondition,
    SimpleTokenScheme,
    StorageDepositReturnUnlockCondition,
    TimelockUnlockCondition,
    ImmutableAccountAddressUnlockCondition,
)

load_dotenv()

client = Client()

ed25519_address = Utils.parse_bech32_address(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

account_address = Utils.parse_bech32_address(
    'rms1pr59qm43mjtvhcajfmupqf23x29llam88yecn6pyul80rx099krmv2fnnux')

address_unlock_condition = AddressUnlockCondition(ed25519_address)

token_scheme = SimpleTokenScheme(50, 0, 100)

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
            ed25519_address,
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
            ed25519_address,
        ),
    ],
)
outputs.append(basic_output)

# Output with immutable account unlock condition
foundry_output = client.build_foundry_output(
    serial_number=1,
    token_scheme=token_scheme,
    unlock_conditions=[
        ImmutableAccountAddressUnlockCondition(
            account_address,
        ),
    ],
)
outputs.append(foundry_output)

print(json.dumps(list(map(lambda o: o.to_dict(), outputs)), indent=2))
