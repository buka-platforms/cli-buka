#!/bin/bash

# Build the workspace in debug mode
cargo build

# Check if build succeeded
if [ $? -eq 0 ]; then
    # Ensure dist_plugins directory exists
    mkdir -p dist_plugins
    # Detect OS and copy the correct library
    if [ "$(uname)" = "Darwin" ]; then
        cp target/debug/libhello.dylib dist_plugins/hello.dylib
        echo "hello.dylib copied to dist_plugins folder."
    else
        cp target/debug/libhello.so dist_plugins/hello.so
        echo "hello.so copied to dist_plugins folder."
    fi
else
    echo "Build failed. Library not copied."
fi