#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Client Client;

typedef struct SecretManager SecretManager;

typedef struct Wallet Wallet;

bool destroy_string(char *ptr);

bool init_logger(const char *config_ptr);

const char *call_utils_method(const char *config_ptr);

const struct Client *create_client(const char *options_ptr);

bool destroy_client(struct Client *client_ptr);

const char *call_client_method(struct Client *client_ptr, char *method_ptr);

const char *binding_get_last_error(void);

const struct SecretManager *create_secret_manager(const char *options_ptr);

bool destroy_secret_manager(struct SecretManager *secret_manager_ptr);

const char *call_secret_manager_method(struct SecretManager *secret_manager, const char *method);

bool destroy_wallet(struct Wallet *wallet_ptr);

const struct Wallet *create_wallet(const char *options_ptr);

const char *call_wallet_method(struct Wallet *wallet_ptr, const char *method_ptr);

bool listen_wallet(struct Wallet *wallet_ptr, const char *events, void (*handler)(const char*));

const struct Client *get_client_from_wallet(struct Wallet *wallet_ptr);

const struct SecretManager *get_secret_manager_from_wallet(struct Wallet *wallet_ptr);
