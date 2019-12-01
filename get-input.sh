#!/bin/bash

inputfile="./input/dec$1.txt"

if ! [ -f $inputfile ]; then
    curl -sSL https://adventofcode.com/2019/day/${1#0}/input --cookie session=$(<session.secret) -o $inputfile
fi
