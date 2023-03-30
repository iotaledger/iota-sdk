import iota_client
from iota_client._node_core_api import NodeCoreAPI
from iota_client._node_indexer_api import NodeIndexerAPI
from iota_client._high_level_api import HighLevelAPI
from iota_client._utils import Utils
from json import dumps
import humps
from datetime import timedelta

class IotaClient(NodeCoreAPI, NodeIndexerAPI, HighLevelAPI, Utils):
    def __init__(
        self,
        nodes=None,
        primary_node = None,
        primary_pow_node = None,
        permanode = None,
        ignore_node_health = None,
        api_timeout = None, 
        node_sync_interval = None,
        remote_pow_timeout = None,
        tips_interval = None,
        quorum = None,
        min_quorum_size = None,
        quorum_threshold = None,
        user_agent = None,
        local_pow = None,
        fallback_to_local_pow = None,
        pow_worker_count = None
    ):
        """Initialize the IOTA Client.

        Parameters
        ----------
        nodes : string or array of string
            A single Node URL or an array of URLs.
        primary_node : string
            Node which will be tried first for all requests.
        primary_pow_node : string
            Node which will be tried first when using remote PoW, even before the primary_node.
        permanode : string
            Permanode URL.
        ignore_node_health : bool
            If the node health should be ignored.
        api_timeout : datetime.timedelta
            Timeout for API requests.
        node_sync_interval : datetime.timedelta
            Interval in which nodes will be checked for their sync status and the [NetworkInfo](crate::NetworkInfo) gets updated.
        remote_pow_timeout : datetime.timedelta
            Timeout when sending a block that requires remote proof of work.
        tips_interval : int
            Tips request interval during PoW in seconds.
        quorum : bool
            If node quorum is enabled. Will compare the responses from multiple nodes 
            and only returns the response if `quorum_threshold`% of the nodes return the same one.
        min_quorum_size : int
            Minimum amount of nodes required for request when quorum is enabled.
        quorum_threshold : int
            % of nodes that have to return the same response so it gets accepted.
        user_agent : string
            The User-Agent header for requests.
        local_pow : bool
            Local proof of work.
        fallback_to_local_pow : bool
            Fallback to local proof of work if the node doesn't support remote PoW.
        pow_worker_count : int
            The amount of threads to be used for proof of work.
        """
        client_config = dict(locals())
        del client_config['self']

        if isinstance(nodes, list):
            _nodes = nodes
            nodes = []
            for node in _nodes:
                if isinstance(node, Node):
                    nodes.append(node.as_dict())
                else:
                    nodes.append(node)
        elif nodes:
            if isinstance(nodes, Node):
                nodes = [nodes.as_dict()]
            else:
                nodes = [nodes]
        client_config['nodes'] = nodes

        client_config = {k:v for k,v in client_config.items() if v != None}

        def get_remaining_nano_seconds(duration: timedelta):
            return (int(duration/timedelta(microseconds=1))-int(duration.total_seconds())*1_000_000)*1_000

        if 'api_timeout' in client_config:
            client_config['api_timeout'] = {'secs': int(client_config['api_timeout'].total_seconds()), 'nanos': get_remaining_nano_seconds(client_config['api_timeout'])}
        if 'node_sync_interval' in client_config:
            client_config['node_sync_interval'] = {'secs': int(client_config['node_sync_interval'].total_seconds()), 'nanos': get_remaining_nano_seconds(client_config['node_sync_interval'])}
        if 'remote_pow_timeout' in client_config:
            client_config['remote_pow_timeout'] = {'secs': int(client_config['remote_pow_timeout'].total_seconds()), 'nanos': get_remaining_nano_seconds(client_config['remote_pow_timeout'])}

        client_config = humps.camelize(client_config)
        client_config = dumps(client_config)

        # Create the message handler
        self.handle = iota_client.create_message_handler(client_config)

    def get_handle(self):
        return self.handle

    def build_alias_output(self,
                           alias_id,
                           unlock_conditions,
                           amount=None,
                           native_tokens=None,
                           state_index=None,
                           state_metadata=None,
                           foundry_counter=None,
                           features=None,
                           immutable_features=None):
        """Build an AliasOutput.
        """
        return self.send_message('buildAliasOutput', {
            'aliasId': alias_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'stateIndex': state_index,
            'stateMetadata': state_metadata,
            'foundryCounter': foundry_counter,
            'features': features,
            'immutableFeatures': immutable_features
        })

    def build_basic_output(self,
                           unlock_conditions,
                           amount=None,
                           native_tokens=None,
                           features=None):
        """Build a BasicOutput.
        """
        return self.send_message('buildBasicOutput', {
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
        })

    def build_foundry_output(self,
                             serial_number,
                             token_scheme,
                             unlock_conditions,
                             amount=None,
                             native_tokens=None,
                             features=None,
                             immutable_features=None):
        """Build a FoundryOutput.
        """
        return self.send_message('buildFoundryOutput', {
            'serialNumber': serial_number,
            'tokenScheme': token_scheme,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        })

    def build_nft_output(self,
                         nft_id,
                         unlock_conditions,
                         amount=None,
                         native_tokens=None,
                         features=None,
                         immutable_features=None):
        """Build an NftOutput.
        """
        return self.send_message('buildNftOutput', {
            'nftId': nft_id,
            'unlockConditions': unlock_conditions,
            'amount': amount,
            'nativeTokens': native_tokens,
            'features': features,
            'immutableFeatures': immutable_features
        })

    def generate_addresses(self, secret_manager, options):
        """Generate addresses.
        """
        return self.send_message('generateAddresses', {
            'secretManager': secret_manager,
            'options': options
        })

    def build_and_post_block(self, secret_manager=None, options=None):
        """Build and post a block.
        """
        return self.send_message('buildAndPostBlock', {
            'secretManager': secret_manager,
            'options': options
        })

    def get_node(self):
        """Get a node candidate from the healthy node pool.
        """
        return self.send_message('getNode')

    def get_network_info(self):
        """Gets the network related information such as network_id and min_pow_score.
        """
        return self.send_message('getNetworkInfo')

    def get_network_id(self):
        """Gets the network id of the node we're connecting to.
        """
        return self.send_message('getNetworkId')

    def get_bech32_hrp(self):
        """Returns the bech32_hrp.
        """
        return self.send_message('getBech32Hrp')

    def get_min_pow_score(self):
        """Returns the min pow score.
        """
        return self.send_message('getMinPowScore')

    def get_tips_interval(self):
        """Returns the tips interval.
        """
        return self.send_message('getTipsInterval')

    def get_local_pow(self):
        """Returns if local pow should be used or not.
        """
        return self.send_message('getLocalPow')

    def get_fall_back_to_local_pow(self):
        """Get fallback to local proof of work timeout.
        """
        return self.send_message('getFallbackToLocalPow')

    def unhealthy_nodes(self):
        """Returns the unhealthy nodes.
        """
        return self.send_message('unhealthyNodes')

    def get_ledger_nano_status(self, is_simulator):
        """Returns the Ledger Status.
        """
        return self.send_message('getLedgerNanoStatus', { 'isSimulator': is_simulator })

    def prepare_transaction(self, secret_manager=None, options=None):
        """Prepare a transaction for signing.
        """
        return self.send_message('prepareTransaction', {
            'secretManager': secret_manager,
            'options': options
        })

    def sign_transaction(self, secret_manager, prepared_transaction_data):
        """Sign a transaction.
        """
        return self.send_message('signTransaction', {
            'secretManager': secret_manager,
            'preparedTransactionData': prepared_transaction_data
        })

    def store_mnemonic(self, secret_manager, mnemonic):
        """Store a mnemonic in the Stronghold vault.
        """
        return self.send_message('storeMnemonic', {
            'secretManager': secret_manager,
            'mnemonic': mnemonic
        })

    def submit_payload(self, payload_dto):
        """Submit a payload in a block.
        """
        return self.send_message('postBlockPayload', {
            'payloadDto': payload_dto
        })

    def sign_ed25519(self, secret_manager, message, chain):
        """Signs a message with an Ed25519 private key.
        """
        return self.send_message('signEd25519', {
            'secretManager': secret_manager,
            'message': message,
            'chain': chain,
        })

    def verify_ed25519_signature(self, signature, message, address):
        """Verifies the Ed25519Signature for a message against an Ed25519Address.
        """
        return self.send_message('verifyEd25519Signature', {
            'signature': signature,
            'message': message,
            'address': address,
        })

class Node():
    def __init__(self, url=None, jwt=None, username=None, password=None, disabled=None):
        """Initialize a Node

        Parameters
        ----------
        url : string
            Node url
        jwt : string
            JWT token
        username : string
            Username for basic authentication
        password : string
            Password for basic authentication
        disabled : bool
            Disable node
        """
        self.url = url
        self.jwt = jwt
        self.username = username
        self.password = password
        self.disabled = disabled

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v}

        if 'jwt' in config or 'username' in config or 'password' in config:
            config['auth'] = {}
            if 'jwt' in config:
                config['auth']['jwt'] = config.pop('jwt')
            if 'username' in config or 'password' in config:
                basic_auth = config['auth']['basic_auth_name_pwd'] = []
                if 'username' in config:
                    basic_auth.append(config.pop('username'))
                if 'password' in config:
                    basic_auth.append(config.pop('password'))

        return config
