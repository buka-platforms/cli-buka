#!/bin/bash

# Set build type (default: debug)
BUILD_TYPE=${1:-debug}

if [ "$BUILD_TYPE" = "release" ]; then
    CARGO_ARGS="build --release"
    BUILD_DIR="target/release"
else
    CARGO_ARGS="build"
    BUILD_DIR="target/debug"
fi

# Build the workspace
cargo $CARGO_ARGS

# Check if build succeeded
if [ $? -eq 0 ]; then
    DIST_DIR="$BUILD_DIR/dist_plugins"
    mkdir -p "$DIST_DIR"

    # Remove all files in dist_plugins directory
    rm -f "$DIST_DIR"/*

    # Detect OS and set library extension
    if [ "$(uname)" = "Darwin" ]; then
        EXT="dylib"
        PREFIX="lib"
    else
        EXT="so"
        PREFIX="lib"
    fi

    # Process all plugins
    for plugin in plugins/*; do
        if [ -d "$plugin" ]; then
            PLUGIN_NAME=$(basename "$plugin")
            LIB_PATH="$BUILD_DIR/${PREFIX}${PLUGIN_NAME}.${EXT}"
            if [ -f "$LIB_PATH" ]; then
                cp "$LIB_PATH" "$DIST_DIR/${PLUGIN_NAME}.${EXT}"
                echo "${PLUGIN_NAME}.${EXT} copied to $DIST_DIR"
            else
                echo "Library not found for plugin '$PLUGIN_NAME': $LIB_PATH"
            fi
        fi
    done
else
    echo "Build failed. Libraries not copied."
fi