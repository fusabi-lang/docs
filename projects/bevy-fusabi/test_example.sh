#!/bin/bash
cd bevy-fusabi
timeout 120s cargo run --example load_script > output.log 2>&1

if grep -q "Script loaded successfully" output.log; then
    echo "SUCCESS: Script loaded and deserialized!"
    cat output.log
    exit 0
else
    echo "FAILURE: Did not find success message."
    cat output.log
    exit 1
fi
