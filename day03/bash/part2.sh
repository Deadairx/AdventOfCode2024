#!/usr/bin/env bash

FILE_PATH=$1

tr -d '\n' < $FILE_PATH |
   sed -E 's/(do\(\)|don'\''t\(\))/\n\1/g' |
   rg -v '^don'\''t()' |
   rg -oP 'mul\(\d+,\d+\)' | 
   sed -E 's/mul\(([0-9]+),([0-9]+)\)/\1*\2/' |
   bc |
   awk '{sum += $1} END {print sum}'
