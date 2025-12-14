#!/bin/bash
cd bevy-fusabi
timeout 120s cargo run --example execute_script > output_exec.log 2>&1

# Expect "Script execution result" output 

if grep -q "Script execution result" output_exec.log; then
    echo "SUCCESS: Script executed!"
    cat output_exec.log
    exit 0
else
    echo "FAILURE: Did not execute script."
    cat output_exec.log
    exit 1
fi
