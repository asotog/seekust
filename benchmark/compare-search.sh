#!/bin/bash

PATH_TO_SEARCH=${1:-.}  # Default to current directory if no argument provided
ITERATIONS=5
total_seekust=0
total_find=0

# Validate path exists
if [ ! -d "$PATH_TO_SEARCH" ]; then
    echo "Error: Directory '$PATH_TO_SEARCH' does not exist"
    exit 1
fi

# Change to target directory
cd "$PATH_TO_SEARCH" || exit 1

echo "Running benchmark ($ITERATIONS iterations) in $(pwd)..."
echo "----------------------------------------"

for i in $(seq 1 $ITERATIONS); do
    echo "Iteration $i:"
    
    # Test seekust console
    start=$(date +%s.%N)
    seekust console > /dev/null
    end=$(date +%s.%N)
    seekust_time=$(echo "$end - $start" | bc)
    total_seekust=$(echo "$total_seekust + $seekust_time" | bc)
    echo "  seekust console: ${seekust_time}s"
    
    # Test find with grep
    start=$(date +%s.%N)
    find . -type f -exec grep -l "console" {} \; > /dev/null
    end=$(date +%s.%N)
    find_time=$(echo "$end - $start" | bc)
    total_find=$(echo "$total_find + $find_time" | bc)
    echo "  find/grep: ${find_time}s"
    echo
done

# Calculate averages
avg_seekust=$(echo "scale=3; $total_seekust / $ITERATIONS" | bc)
avg_find=$(echo "scale=3; $total_find / $ITERATIONS" | bc)

echo "Results:"
echo "----------------------------------------"
echo "Average seekust console: ${avg_seekust}s"
echo "Average find/grep: ${avg_find}s"

# Calculate percentage difference
if (( $(echo "$avg_seekust > $avg_find" | bc -l) )); then
    pct_diff=$(echo "scale=1; ($avg_seekust/$avg_find - 1) * 100" | bc)
    echo "seekust is ${pct_diff}% slower than find/grep"
else
    pct_diff=$(echo "scale=1; (1 - $avg_seekust/$avg_find) * 100" | bc)
    echo "seekust is ${pct_diff}% faster than find/grep"
fi
