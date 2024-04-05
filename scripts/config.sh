#!/bin/bash

# Root directory.
CC_PROJECT_PATH="$(dirname "$(pwd)")"

# CMake build files and cache.
export CC_BUILD_DIR="${CC_PROJECT_PATH}/build"

# Binary directory.
export CC_BIN_DIR="${CC_PROJECT_PATH}/bin"
