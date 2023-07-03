#!/usr/bin/env bash
set -euo pipefail

cbindgen --crate iota-sdk-native --output headers/iota_sdk.h --lang c
cbindgen --crate iota-sdk-native --output headers/iota_sdk.hpp --lang c++