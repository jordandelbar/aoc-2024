#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <run|test>"
    exit 1
fi

if [ "$1" != "run" ] && [ "$1" != "test" ] && [ "$1" != "clean" ] && [ "$1" != "clippy" ]; then
    echo "Invalid argument. Use 'run' or 'test'."
    exit 1
fi

for dir in day_*/; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Processing $dir"
        (
            cd "$dir" || exit
            cargo "$1"
        )
    else
        echo "Skipping $dir (no Cargo.toml found)"
    fi
done
