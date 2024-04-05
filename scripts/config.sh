#!/bin/bash

# Root directory.
CC_PROJECT_PATH="$(dirname "$(pwd)")"

# Docs directory.
export CC_DOCS_DIR="${CC_PROJECT_PATH}/docs/html"

# CMake build files and cache.
export CC_BUILD_DIR="${CC_PROJECT_PATH}/build"

# Binary directory.
export CC_BIN_DIR="${CC_PROJECT_PATH}/bin"
