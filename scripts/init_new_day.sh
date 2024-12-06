#!/usr/bin/env bash

day_number=$1
dir_name="day$day_number"
cargo new "$dir_name"
cd "$dir_name"
touch .gitignore
# Adding this to a gitignore since I'm using JetBrains
echo "/.idea" >> .gitignore
echo "/target" >> .gitignore
touch example.txt
touch input.txt
open -na "RustRover.app"
