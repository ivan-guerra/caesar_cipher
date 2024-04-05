#!/bin/bash

source config.sh

# Remove the CMake build directory.
if [ -d "$CC_BUILD_DIR" ]
then
    echo "removing '$CC_BUILD_DIR'"
    rm -rf "$CC_BUILD_DIR"
fi

# Remove the binary directory.
if [ -d "$CC_BIN_DIR" ]
then
    echo "removing '$CC_BIN_DIR'"
    rm -rf "$CC_BIN_DIR"
fi
