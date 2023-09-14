#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Client;

struct SecretManager;

struct Wallet;

extern "C" {

bool destroy_string(char *ptr);

bool init_logger(const char *config_ptr);

const char *call_utils_method(const char *config_ptr);

const Client *create_client(const char *options_ptr);

bool destroy_client(Client *client_ptr);

const char *call_client_method(Client *client_ptr, char *method_ptr);

const char *binding_get_last_error();

const SecretManager *create_secret_manager(const char *options_ptr);

bool destroy_secret_manager(SecretManager *secret_manager_ptr);

const char *call_secret_manager_method(SecretManager *secret_manager, const char *method);

bool destroy_wallet(Wallet *wallet_ptr);

const Wallet *create_wallet(const char *options_ptr);

const char *call_wallet_method(Wallet *wallet_ptr, const char *method_ptr);

bool listen_wallet(Wallet *wallet_ptr, const char *events, void (*handler)(const char*));

const Client *get_client_from_wallet(Wallet *wallet_ptr);

const SecretManager *get_secret_manager_from_wallet(Wallet *wallet_ptr);

} // extern "C"
