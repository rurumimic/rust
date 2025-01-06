#!/bin/bash

# Define the termination patterns
pattern1="x = 10, y =  0"
pattern2="x =  0, y = 20"

# Set timeout (30 seconds)
end_time=$((SECONDS + 30))

while [ $SECONDS -lt $end_time ]; do
    # Execute the command and capture its output
    output=$(./target/release/relaxed)
    echo "$output"

    # Check if the output matches either pattern
    if [[ "$output" == *"$pattern1"* || "$output" == *"$pattern2"* ]]; then
        echo "Condition met: $output"
        exit 0
    fi

    # Prevent excessive execution by adding a short delay
    # sleep 0.0001
done

# If the condition is not met within 30 seconds, print a timeout message
echo "Condition not met within 30 seconds."
exit 1

