#!/usr/bin/env bash

FILE_PATH=$1

awk '{count1[$1]++; count2[$2]++} END {for (x in count1) if (x in count2) sum += x * count2[x] * count1[x]; print sum}' $FILE_PATH
