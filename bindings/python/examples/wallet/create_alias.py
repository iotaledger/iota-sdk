from iota_sdk import Wallet, init_logger
from dotenv import load_dotenv
import json
import os

load_dotenv()

log_config = {
    "name": './wallet.log',
    "levelFilter": 'debug',
    "targetExclusions": ["h2", "hyper", "rustls"]
}

# Init the logger
init_logger(json.dumps(log_config))

# In this example we will create an alias ouput

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")    

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.prepare_create_alias_output(None, None).finish()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
