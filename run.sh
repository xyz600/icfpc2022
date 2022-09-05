#!/bin/bash

set -eu

cargo build --release --bin solver
cp target/release/solver ./solver_bin

# single experiment
dataset="`seq 1 25` `seq 36 40`"
echo $dataset
parallel --progress --result result ./solver_bin -i {} -s 6 ::: $dataset &

# twin experiment
dataset_twin=`seq 26 35`
parallel --progress ./solver_bin -i {} -s 5 -t ::: $dataset_twin &

wait

rm ./solver_bin
