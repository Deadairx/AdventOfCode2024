#!/usr/bin/env bash

FILE_PATH=$1

rg -oP 'mul\(\d+,\d+\)' $FILE_PATH | 
   sed -E 's/mul\(([0-9]+),([0-9]+)\)/\1*\2/' |
   bc |
   awk '{sum += $1} END {print sum}'

