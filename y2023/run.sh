#!/bin/bash

DAY=$1

gcc -O3 day${DAY}.c -o day${DAY}.out && ./day${DAY}.out