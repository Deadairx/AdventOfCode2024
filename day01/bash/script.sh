#!/usr/bin/env bash

FILE_PATH=$1

paste <(awk '{print $1}' "$FILE_PATH" | sort) <(awk '{print $2}' "$FILE_PATH" | sort) | 
	awk '{diff = $1 - $2; if (diff < 0) diff = -diff; sum += diff} END {print sum}' > result.txt

