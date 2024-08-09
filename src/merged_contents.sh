#!/bin/bash

# Define the output file in the current directory
output_file="./merged_contents.txt"

# Empty the output file if it already exists
> "$output_file"

# Function to process files
process_file() {
  local file=$1
  local extension=${file##*.}
  
  if [[ "$extension" == "rs" || "$extension" == "js" || "$extension" == "css" ]]; then
    echo "${file##*/}:" >> "$output_file"
    cat "$file" >> "$output_file"
    echo -e "\n\n\n" >> "$output_file"
  fi
}

# Export function for find
export -f process_file
export output_file

# Find and process all files
find . -type f -exec bash -c 'process_file "$0"' {} \;

# Append tree output
echo "Directory structure:" >> "$output_file"
tree . >> "$output_file"
