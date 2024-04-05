#!/bin/bash

# Script usage: ./build.sh [RELEASE|DEBUG]

source config.sh

Main()
{
    # Create the build directory if it does not already exist.
    mkdir -pv "$CC_BUILD_DIR"

    # Set the build type according to the first program arg if any. Default is release.
    BUILD_TYPE="RELEASE"
    if [ -n  "$1" ]
    then
        BUILD_TYPE=$1
    fi

    pushd "$CC_BUILD_DIR" > /dev/null || exit 1
    cmake ../ \
        -DBUILD_DOCS=ON \
        -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
        -DCMAKE_INSTALL_PREFIX="$CC_BIN_DIR" \
        -DCMAKE_BUILD_TYPE="$BUILD_TYPE" && \
        make -j"$(nproc)" all && \
        make install

    # Exit if any of the above commands fails.
    if [ $? -ne 0 ];
    then
        exit 1
    fi
    popd > /dev/null || exit 1
}

Main "$1"
