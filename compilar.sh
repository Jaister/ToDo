#!/bin/bash
if [ "$#" -eq 0 ]; then
    echo "No arguments provided."
    echo "Usage: $0 arg1 arg2 ..."
    exit 1
fi

# Iterate over the arguments
if [ "$1" == "all" ]; then
    echo "Compiling all"
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-pc-windows-gnu
    exit 0
elif [ "$1" == "linux" ]; then
    echo "Compiling for linux"
    cargo build --release --target x86_64-unknown-linux-gnu
    exit 0
elif [ "$1" == "windows" ]; then
    echo "Compiling for windows"
    cargo build --release --target x86_64-pc-windows-gnu
    exit 0
else
    echo "Invalid argument"
    exit 1
fi