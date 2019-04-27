#!/usr/bin/bash
for p in 1 2 3 ; do
    for seed in 2048 1024 512 ; do
        echo $p
        echo $seed
        ./generator.out testcase.in $p $seed
        python3 B.py
        ./output_checker.out testcase.in result.out $seed
    done
done