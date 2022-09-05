#!/bin/bash

set -eu

SINGLE_MAX_ID=25
TWIN_MAX_ID=35

cargo build --release --bin solver
cp target/release/solver ./solver_bin

# single experiment
dataset="`seq 4 25` `seq 36 40`"
echo $dataset
parallel --progress --result result ./solver_bin -i {} -s 6 ::: $dataset &

# twin experiment
dataset_twin=`seq 26 35`
parallel --progress ./solver_bin -i {} -s 8 -t ::: $dataset_twin &

wait

rm ./solver_bin
